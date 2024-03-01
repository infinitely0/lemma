use {
    crate::{
        errors,
        errors::CompilerError,
        grammar::{
            self, Argument, Call, Definition, Element, Expression, Function, FunctionCall,
            Operation, Pipe, Program, Signature,
        },
        parser::Parser,
        tokens::{
            Conditional, Identifier, LocatedIdentifier, Operator, Symbol, Token, Type, Value,
        },
    },
    grammar::List,
    std::collections::BTreeMap,
};

pub fn build(tokens: Vec<Token>) -> Result<Program, CompilerError> {
    if tokens.is_empty() {
        return Err(CompilerError::Parser("empty program".to_string(), 0));
    }
    let mut parser = Parser::new(tokens);
    let mut program: Program = Program {
        functions: Vec::new(),
    };

    while parser.has_more() {
        // Allow blank lines between functions
        if let &Token::Symbol(Symbol::EOL, _) = parser.peek() {
            parser.advance();
            continue;
        }
        let function = parse_function(&mut parser)?;
        program.functions.push(function);
    }

    Ok(program)
}

fn parse_function(parser: &mut Parser) -> Result<Function, CompilerError> {
    let signature = parse_signature(parser)?;
    let definition = parse_definition(parser)?;

    Ok(Function {
        signature,
        definition,
    })
}

fn parse_signature(parser: &mut Parser) -> Result<Signature, CompilerError> {
    let mut parameters: Vec<Type> = Vec::new();
    let line = parser.location();
    while !matches!(parser.peek(), Token::Symbol(Symbol::Return, _)) {
        match parser.advance() {
            Token::Type(t, _) => parameters.push(t.clone()),
            Token::Symbol(Symbol::LB, _) => {
                // Parse list parameter
                if let Token::Type(t, _) = parser.advance() {
                    parameters.push(Type::List(Box::new(t.clone())));

                    if !matches!(parser.advance(), Token::Symbol(Symbol::RB, _)) {
                        return Err(CompilerError::Parser(
                            "expected closing bracket in function signature parameters".into(),
                            line,
                        ));
                    }
                } else {
                    return Err(CompilerError::Parser(
                        "expected type in list parameter in function signature".into(),
                        line,
                    ));
                }
            }
            t => {
                return Err(CompilerError::Parser(
                    format!("unexpected token in function signature: `{}`", t),
                    line,
                ));
            }
        }
    }

    parser.advance(); // Consume `->`

    let returns = match parser.advance() {
        Token::Type(t, _) => t.clone(),
        Token::Symbol(Symbol::LB, _) => {
            // Parse list parameter
            if let Token::Type(t, _) = parser.advance() {
                let ret = Type::List(Box::new(t.clone()));

                if !matches!(parser.advance(), Token::Symbol(Symbol::RB, _)) {
                    return Err(CompilerError::Parser(
                        "expected closing bracket in function return type".into(),
                        line,
                    ));
                }

                ret
            } else {
                return Err(CompilerError::Parser(
                    "expected type in list parameter in function signature return type".into(),
                    parser.location(),
                ));
            }
        }
        _ => {
            return Err(CompilerError::Parser(
                "expected return type in function signature".into(),
                parser.location(),
            ));
        }
    };

    if !matches!(parser.advance(), Token::Symbol(Symbol::EOL, _)) {
        return Err(CompilerError::Parser(
            "expected function definition after signature".into(),
            parser.location(),
        ));
    }

    Ok(Signature {
        parameters,
        returns,
    })
}

fn parse_definition(parser: &mut Parser) -> Result<Definition, CompilerError> {
    let name = match parser.advance() {
        Token::Identifier(name, _) => Ok(name.clone()),
        Token::Symbol(Symbol::EOL, _) => Err(CompilerError::Parser(
            "expected definition name, found empty line".into(),
            parser.location() - 1,
        )),
        token => Err(CompilerError::Parser(
            format!("expected definition name, found `{}`", token),
            parser.location(),
        )),
    }?;
    let line = parser.location();
    let parameters = parse_params(parser)?;
    let body = parse_expression(parser)?;
    let name = LocatedIdentifier { id: name, line };
    Ok(Definition {
        name,
        parameters,
        body,
    })
}

fn parse_params(parser: &mut Parser) -> Result<Vec<LocatedIdentifier>, CompilerError> {
    let mut params: BTreeMap<Identifier, LocatedIdentifier> = BTreeMap::new();
    while !matches!(parser.peek(), Token::Symbol(Symbol::Assign, _)) {
        if let Token::Identifier(l_id, _) = parser.advance() {
            let l_id = LocatedIdentifier {
                id: l_id.clone(),
                line: parser.location(),
            };
            if params.contains_key(&l_id.id) {
                return Err(CompilerError::Parser(
                    format!("duplicate parameter name `{}`", l_id.id),
                    l_id.line,
                ));
            }
            params.insert(l_id.id.clone(), l_id);
        } else {
            return Err(CompilerError::Parser(
                "expected identifier in definition parameters".into(),
                parser.location(),
            ));
        }
    }
    parser.advance();
    Ok(params.into_values().collect())
}

fn parse_expression(parser: &mut Parser) -> Result<Expression, CompilerError> {
    let expression = match parser.peek() {
        Token::Value(_, _) => parse_value(parser),
        Token::Identifier(_, _) => parse_call(parser),
        Token::Operator(_, _) => parse_operation(parser),
        Token::Conditional(Conditional::If, _) => parse_conditional(parser),
        Token::Type(_, _) => parse_list(parser),
        token => Err(CompilerError::Parser(
            format!("expected expression, found {}", token),
            parser.location(),
        )),
    };
    if let Token::Symbol(Symbol::Pipe, _) = parser.peek() {
        parse_pipe(parser, expression?)
    } else {
        expression
    }
}

fn parse_pipe(parser: &mut Parser, expression: Expression) -> Result<Expression, CompilerError> {
    if !matches!(parser.advance(), Token::Symbol(Symbol::Pipe, _)) {
        unreachable!("expected `>>`");
    }
    let pipe = match parser.advance() {
        Token::Identifier(i, l) => Pipe {
            left: Box::new(expression),
            right: i.clone(),
            line: *l,
        },
        token => {
            return Err(CompilerError::Parser(
                format!("expected identifier after pipe, found `{}`", token),
                parser.location(),
            ))
        }
    };
    Ok(Expression::Call(Call::Pipe(pipe)))
}

fn parse_value(parser: &mut Parser) -> Result<Expression, CompilerError> {
    match parser.advance() {
        Token::Value(value, _) => Ok(Expression::Value(value.clone())),
        _ => unreachable!(),
    }
}

fn parse_call(parser: &mut Parser) -> Result<Expression, CompilerError> {
    let name = match parser.advance() {
        Token::Identifier(name, _) => name.clone(),
        _ => unreachable!(),
    };
    let line = parser.location();
    let name = LocatedIdentifier { id: name, line };
    let arguments: Vec<Argument> = parse_arguments(parser)?;
    Ok(Expression::Call(Call::FunctionCall(FunctionCall {
        name,
        arguments,
        line,
    })))
}

fn parse_operation(parser: &mut Parser) -> Result<Expression, CompilerError> {
    let line = parser.location();
    let operator = match parser.advance() {
        Token::Operator(operator, _) => operator.clone(),
        _ => unreachable!(),
    };
    let arguments: Vec<Argument> = parse_arguments(parser)?;
    Ok(Expression::Call(Call::Operation(Operation {
        operator,
        arguments,
        line,
    })))
}

fn parse_list(parser: &mut Parser) -> Result<Expression, CompilerError> {
    let list_type = match parser.advance() {
        Token::Type(t, _) => t.clone(),
        _ => {
            return Err(CompilerError::Parser(
                "expected type in list expression".into(),
                parser.location(),
            ));
        }
    };

    if !matches!(parser.advance(), Token::Symbol(Symbol::LB, _)) {
        return Err(errors::unexpected_token(
            "opening bracket",
            parser.location(),
        ));
    }

    let line = parser.location();
    let mut elements: Vec<Element> = Vec::new();

    // Empty list
    if matches!(parser.peek(), Token::Symbol(Symbol::RB, _)) {
        parser.advance(); // Consume closing bracket
        let list = List {
            list_type,
            elements,
            line,
        };
        return Ok(Expression::Value(Value::List(list)));
    }

    parse_list_body(parser, &list_type, &mut elements)?;

    let list = List {
        list_type,
        elements,
        line,
    };
    Ok(Expression::Value(Value::List(list)))
}

fn parse_list_body(
    parser: &mut Parser,
    list_type: &Type,
    elements: &mut Vec<Element>,
) -> Result<(), CompilerError> {
    let first_element = match parser.advance() {
        Token::Value(value, _) => Element::Value(value.clone()),
        Token::Identifier(id, _) => Element::Identifier(id.clone()),
        token => {
            return Err(CompilerError::Parser(
                format!("unexpected token in list: `{}`", token),
                parser.location(),
            ));
        }
    };

    // Parse range
    if matches!(parser.peek(), Token::Symbol(Symbol::Range, _)) {
        if *list_type != Type::Int {
            return Err(CompilerError::Parser(
                "range syntax is only allowed for lists of integers".into(),
                parser.location(),
            ));
        }

        let start_value = if let Element::Value(Value::Integer(n)) = first_element {
            n
        } else {
            return Err(CompilerError::Parser(
                "range start must be an integer".into(),
                parser.location(),
            ));
        };

        parser.advance(); // Consume ..

        let end_value = match parser.advance() {
            Token::Value(Value::Integer(n), _) => n,
            token => {
                return Err(CompilerError::Parser(
                    format!("expected integer after '..', found `{}`", token),
                    parser.location(),
                ));
            }
        };

        if start_value <= *end_value {
            for n in start_value..=*end_value {
                elements.push(Element::Value(Value::Integer(n)));
            }
        } else {
            for n in (*end_value..=start_value).rev() {
                elements.push(Element::Value(Value::Integer(n)));
            }
        }

        if !matches!(parser.advance(), Token::Symbol(Symbol::RB, _)) {
            return Err(errors::unexpected_token(
                "closing bracket",
                parser.location(),
            ));
        }
    } else {
        // Not a range
        elements.push(first_element);
        parse_list_elements(parser, elements)?;

        if !matches!(parser.advance(), Token::Symbol(Symbol::RB, _)) {
            return Err(errors::unexpected_token(
                "closing bracket",
                parser.location(),
            ));
        }
    }

    Ok(())
}

fn parse_list_elements(
    parser: &mut Parser,
    elements: &mut Vec<Element>,
) -> Result<(), CompilerError> {
    loop {
        if matches!(parser.peek(), Token::Symbol(Symbol::RB, _)) {
            break;
        }

        let element = match parser.advance() {
            Token::Value(value, _) => Element::Value(value.clone()),
            Token::Identifier(id, _) => Element::Identifier(id.clone()),
            Token::Symbol(Symbol::EOF, _) => {
                return Err(errors::unexpected_token(
                    "closing bracket",
                    parser.location(),
                ));
            }
            token => {
                return Err(CompilerError::Parser(
                    format!("unexpected token in list: `{}`", token),
                    parser.location(),
                ));
            }
        };
        elements.push(element);
    }
    Ok(())
}

fn parse_arguments(parser: &mut Parser) -> Result<Vec<Argument>, CompilerError> {
    let mut args: Vec<Argument> = Vec::new();
    while parser.has_more() {
        let line = parser.location();
        let arg = match parser.peek() {
            Token::Operator(op, _) => {
                if op == &Operator::Sub || op == &Operator::Add {
                    parse_operation(parser).map(Argument::ParenExpression)
                } else {
                    Err(CompilerError::Parser(
                        format!("unexpected token in function arguments: `{}`", op),
                        parser.location(),
                    ))
                }
            }
            Token::Value(_, _) => match parser.advance() {
                Token::Value(value, _) => Ok(Argument::Value(value.clone())),
                _ => unreachable!(),
            },
            Token::Identifier(_, _) => match parser.advance() {
                Token::Identifier(id, _) => {
                    let l_id = LocatedIdentifier {
                        id: id.clone(),
                        line,
                    };
                    Ok(Argument::Identifier(l_id))
                }
                _ => unreachable!(),
            },
            Token::Symbol(Symbol::LP, _) => parse_paren_expression(parser),
            Token::Symbol(Symbol::RP, _) => break,
            Token::Symbol(Symbol::Pipe, _) => break,
            Token::Type(_, _) => {
                if let Ok(Expression::Value(Value::List(list))) = parse_list(parser) {
                    Ok(Argument::Value(Value::List(list)))
                } else {
                    Err(CompilerError::Parser(
                        "expected list expression".into(),
                        parser.location(),
                    ))
                }
            }
            Token::Symbol(Symbol::EOL, _) => {
                parser.advance();
                break;
            }
            Token::Conditional(Conditional::Then, _) => break,
            Token::Conditional(Conditional::Else, _) => break,
            token => Err(CompilerError::Parser(
                format!("unexpected token in function arguments: `{}`", token),
                parser.location(),
            )),
        }?;
        args.push(arg);
    }
    Ok(args)
}

fn parse_conditional(parser: &mut Parser) -> Result<Expression, CompilerError> {
    let line = parser.location();
    if !matches!(parser.advance(), Token::Conditional(Conditional::If, _)) {
        unreachable!("expected `if`");
    }
    let condition = parse_expression(parser)?;

    // Allow `then` on new line
    if let &Token::Symbol(Symbol::EOL, _) = parser.peek() {
        parser.advance();
    }
    if !matches!(parser.advance(), Token::Conditional(Conditional::Then, _)) {
        return Err(errors::unexpected_token("then", line));
    }

    let then = parse_expression(parser)?;

    // Allow `else` on new line
    if let &Token::Symbol(Symbol::EOL, _) = parser.peek() {
        parser.advance();
    }
    let p = parser.advance();
    if !matches!(p, Token::Conditional(Conditional::Else, _)) {
        println!("{:?}", p);
        return Err(errors::unexpected_token("else", parser.location()));
    }
    let otherwise = parse_expression(parser)?;

    let conditional = grammar::Conditional {
        condition: Box::new(condition),
        then: Box::new(then),
        otherwise: Box::new(otherwise),
        line,
    };
    Ok(Expression::Conditional(conditional))
}

fn parse_paren_expression(parser: &mut Parser) -> Result<Argument, CompilerError> {
    if !matches!(parser.advance(), Token::Symbol(Symbol::LP, _)) {
        unreachable!("expected opening parenthesis");
    }
    let expression = parse_expression(parser)?;
    if !matches!(parser.advance(), Token::Symbol(Symbol::RP, _)) {
        return Err(CompilerError::Parser(
            "expected closing parenthesis".into(),
            parser.location(),
        ));
    }
    Ok(Argument::ParenExpression(expression))
}
