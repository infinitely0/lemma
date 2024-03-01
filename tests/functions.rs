use lemma::{ast, interpreter, lexer, log::exit, tokens::Value::Integer};

#[test]
fn remainder_function() {
    let source = "-> Void
                        main = rem 10 7

                        Int Int -> Int
                        rem a b = - a (* b (/ a b))";

    let tokens = lexer::tokens(&source).unwrap_or_else(|err| exit(err));
    let program = ast::build(tokens).unwrap_or_else(|err| exit(err));

    let output = interpreter::evaluate(program).unwrap().unwrap();

    assert_eq!(output, Integer(3));
}
