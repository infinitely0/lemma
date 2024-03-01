use std::fs;

use lemma::{self, ast, lexer, log::exit};

#[test]
fn hello_world() {
    let source = fs::read_to_string("examples/hello-world.lm")
        .unwrap()
        .to_string();
    let tokens = lexer::tokens(&source).unwrap_or_else(|err| exit(err));
    let program = ast::build(tokens).unwrap_or_else(|err| exit(err));
    println!("{:#?}", program);
}
