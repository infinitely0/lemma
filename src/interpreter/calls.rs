use {
    crate::{
        errors,
        errors::CompilerError,
        grammar::{
            Argument, Call, Definition, Element, Expression, Function, FunctionCall, List, Pipe,
            Signature,
        },
        interpreter::{arguments, environment::Environment, functions, operations},
        tokens::{Identifier, LocatedIdentifier, Value},
    },
    std::{cell::RefCell, collections::HashMap, rc::Rc},
};

pub fn eval(
    env: Rc<RefCell<Environment>>,
    fc: &FunctionCall,
) -> Result<Option<Value>, CompilerError> {
    let function = env
        .borrow()
        .get(&fc.name.id)
        .ok_or_else(|| errors::undefined_variable(&fc.name.id, fc.line))?;

    let params = &function.definition.parameters;

    let bindings: HashMap<Identifier, Function> = params
        .iter()
        .zip(fc.arguments.iter())
        .map(|(id, arg)| {
            let value = arguments::eval(Rc::clone(&env), arg)?;
            let function = to_fn(value);
            Ok((id.clone().id, function))
        })
        .collect::<Result<_, _>>()?;

    let env = Environment::with_enclosing(bindings, Rc::clone(&env));

    let result = functions::eval(Rc::new(RefCell::new(env)), &function)?;
    Ok(result)
}

pub fn eval_pipe(
    env: Rc<RefCell<Environment>>,
    pipe: &Pipe,
) -> Result<Option<Value>, CompilerError> {
    let arg = match *pipe.left {
        Expression::Value(ref v) => Ok(v.clone()),
        Expression::Call(ref call) => {
            let result = match call {
                Call::Operation(op) => operations::eval(Rc::clone(&env), op),
                Call::FunctionCall(fc) => eval(Rc::clone(&env), fc),
                Call::Pipe(p) => eval_pipe(Rc::clone(&env), p),
            };

            result?.ok_or_else(|| {
                CompilerError::Interpreter(
                    "Expected expression on left side of pipe, found none".to_string(),
                    pipe.line,
                )
            })
        }
        _ => {
            return Err(CompilerError::Interpreter(
                "Expected expression on left side of pipe".to_string(),
                pipe.line,
            ));
        }
    }?;

    if let Value::List(ref list) = arg {
        let mut resolved: Vec<Value> = Vec::new();
        for elem in list.elements.iter() {
            let v = match elem {
                Element::Value(v) => v.clone(),
                Element::Identifier(id) => env
                    .borrow()
                    .get(id)
                    .map(|f| functions::eval(Rc::clone(&env), &f))
                    .ok_or_else(|| errors::undefined_variable(id, pipe.line))??
                    .ok_or_else(|| errors::undefined_argument(id, pipe.line))?,
            };
            resolved.push(v);
        }

        let list = resolved
            .iter()
            .map(|v| {
                let function_call = FunctionCall {
                    name: LocatedIdentifier {
                        id: pipe.right.clone(),
                        line: pipe.line,
                    },
                    arguments: vec![Argument::Value(v.clone())],
                    line: pipe.line,
                };
                eval(Rc::clone(&env), &function_call)
            })
            .collect::<Result<Vec<Option<Value>>, CompilerError>>()?;

        return Ok(Some(Value::List(List {
            list_type: arg.get_type(),
            elements: list
                .into_iter()
                .map(|v| Element::Value(v.unwrap()))
                .collect(),
            line: pipe.line,
        })));
    }

    let function_call = FunctionCall {
        name: LocatedIdentifier {
            id: pipe.right.clone(),
            line: pipe.line,
        },
        arguments: vec![Argument::Value(arg)],
        line: pipe.line,
    };
    eval(Rc::clone(&env), &function_call)
}

// 🤔
fn to_fn(value: Value) -> Function {
    Function {
        signature: Signature {
            parameters: vec![],
            returns: value.get_type(),
        },
        definition: Definition {
            name: LocatedIdentifier {
                id: value.to_string(),
                line: 0,
            },
            parameters: vec![],
            body: Expression::Value(value),
        },
    }
}
