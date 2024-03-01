use lemma::{
    errors::CompilerError,
    lexer,
    tokens::{Operator, Symbol, Token, Type, Value},
};

#[test]
fn signature_lexed() {
    let source = "Int -> Str";
    let tokens = lexer::tokens(source);
    let expected = vec![
        Token::Type(Type::Int, 1),
        Token::Symbol(Symbol::Return, 1),
        Token::Type(Type::Str, 1),
    ];
    assert_eq!(expected, tokens.unwrap());
}

#[test]
fn invalid_character_detected() {
    let source = "Int @";
    let tokens = lexer::tokens(source);
    assert!(matches!(tokens, Err(CompilerError::Lexer(_, _))));
}

#[test]
fn valid_characters_accepted() {
    let source = "a 42 \"a\" 3.1 true";
    let tokens = lexer::tokens(source);
    assert!(tokens.is_ok());
    let expected = vec![
        Token::Identifier("a".to_string(), 1),
        Token::Value(Value::Integer(42), 1),
        Token::Value(Value::String("a".to_string()), 1),
        Token::Value(Value::Fractional(3.1), 1),
        Token::Value(Value::Boolean(true), 1),
    ];
    assert_eq!(expected, tokens.unwrap());
}

#[test]
fn whitespace_ignored() {
    let source = "  Int  ->  Int  ";
    let tokens = lexer::tokens(source);
    let expected = vec![
        Token::Type(Type::Int, 1),
        Token::Symbol(Symbol::Return, 1),
        Token::Type(Type::Int, 1),
    ];
    assert_eq!(expected, tokens.unwrap());
}

#[test]
fn print_program_from_tokens() {
    let tokens = vec![
        Token::Type(Type::Int, 1),
        Token::Symbol(Symbol::Return, 1),
        Token::Type(Type::Int, 1),
        Token::Symbol(Symbol::EOL, 1),
        Token::Identifier("add".to_string(), 2),
        Token::Identifier("a".to_string(), 2),
        Token::Symbol(Symbol::Assign, 2),
        Token::Operator(Operator::Add, 2),
        Token::Identifier("a".to_string(), 2),
        Token::Value(Value::Integer(1), 2),
        Token::Symbol(Symbol::EOL, 2),
    ];

    let tokens: Vec<String> = tokens.into_iter().map(|t| t.into()).collect();
    let program = tokens.join(" ").replace(" \n ", "\n");
    let str = "Int -> Int\nadd a = + a 1";
    assert_eq!(str, program.trim());
}
