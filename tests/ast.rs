use lemma::{ast, errors::CompilerError, lexer};

#[test]
fn multiple_functions_parsed() {
    let source = "
        Int -> Int
        add-one a = + a 1

        Int -> Int
        add-two a = + a 2";
    let tokens = lexer::tokens(source).unwrap();
    let program = ast::build(tokens);
    assert!(program.is_ok());
    assert_eq!(program.unwrap().functions.len(), 2);
}

#[test]
fn parens_matched() {
    let source = "
        Int -> Int
        foo a = + (+ a a) (+ 1 1)";
    let tokens = lexer::tokens(source).unwrap();
    let program = ast::build(tokens);
    assert!(program.is_ok());
}

#[test]
fn nested_parens_matched() {
    let source = "
        Int -> Int
        foo a = + (+ a (- 1 a)) (+ 1 1)";
    let tokens = lexer::tokens(source).unwrap();
    let program = ast::build(tokens);
    assert!(program.is_ok());
}

#[test]
fn missing_right_paren_detected() {
    let source = "
        Int -> Int
        foo a = + 1 (+ 1 1";
    let tokens = lexer::tokens(source).unwrap();
    let program = ast::build(tokens);
    assert!(matches!(program, Err(CompilerError::Parser(_, _))));
    assert!(program
        .unwrap_err()
        .to_string()
        .contains("expected closing parenthesis"));
}

#[test]
fn missing_left_paren_detected() {
    let source = "Int -> Int \nfoo a = + 1)";
    let tokens = lexer::tokens(source).unwrap();
    let program = ast::build(tokens);
    assert!(matches!(program, Err(CompilerError::Parser(_, _))));
}
