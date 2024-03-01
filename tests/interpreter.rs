use lemma::{ast, errors::CompilerError, interpreter, lexer, log::exit};

#[test]
fn main_not_found() {
    let source = "Int -> Int ; add a = + a 1";
    let tokens = lexer::tokens(source).unwrap_or_else(|err| exit(err));
    let program = ast::build(tokens).unwrap_or_else(|err| exit(err));
    let output = interpreter::evaluate(program);
    assert!(matches!(output, Err(CompilerError::Interpreter(_, _))));
}

#[test]
fn arity_matches() {
    let source = "Int -> Int ; main a = + 1 1";
    let tokens = lexer::tokens(source).unwrap_or_else(|err| exit(err));
    let program = ast::build(tokens).unwrap_or_else(|err| exit(err));
    let output = interpreter::evaluate(program);
    assert!(output.is_ok())
}

#[test]
fn arity_mismatch_sig() {
    let source = "Int Int -> Int ; main a = + a 1";
    let tokens = lexer::tokens(source).unwrap_or_else(|err| exit(err));
    let program = ast::build(tokens).unwrap_or_else(|err| exit(err));
    let output = interpreter::evaluate(program);
    assert!(matches!(output, Err(CompilerError::Interpreter(_, _))));
}

#[test]
fn arity_mismatch_args() {
    let source = "Int -> Int ; a b = + a 1";
    let tokens = lexer::tokens(source).unwrap_or_else(|err| exit(err));
    let program = ast::build(tokens).unwrap_or_else(|err| exit(err));
    let output = interpreter::evaluate(program);
    assert!(matches!(output, Err(CompilerError::Interpreter(_, _))));
}
