use std::{env, fmt::Write};

use lemma::{
    args::Args,
    ast, interpreter, lexer,
    log::{self, env_log_level, exit, exit_with_info},
    printer,
    tokens::{Symbol, Token},
};

fn main() {
    log::debug("Starting application");
    log::debug(&format!("Log level set to {}", env_log_level()));

    log::debug("Parsing arguments...");
    let args = Args::build(&mut env::args()).unwrap_or_else(|err| exit(err));
    log::debug(&format!("{:?}", args));

    log::debug(&format!("Reading source code from {}", args.file_path));
    let source = args.source().unwrap_or_else(|err| exit(err));
    log::debug(&format!("Source:\n{}", source));

    log::debug("Starting lexical analysis...");
    let tokens = lexer::tokens(&source).unwrap_or_else(|err| exit_with_info(err, &source));
    let tokens_str = tokens
        .iter()
        .filter(|t| !matches!(*t, Token::Symbol(Symbol::EOL, _)))
        .fold(String::new(), |mut acc, t| {
            writeln!(acc, "{:?}", t).unwrap();
            acc
        });
    log::debug(&format!("Tokens:\n{}", tokens_str));

    log::debug("Building AST...");
    let program = ast::build(tokens).unwrap_or_else(|err| exit_with_info(err, &source));
    let json = serde_json::to_value(&program).unwrap();
    let tree = printer::pretty_print_ast(json);
    log::debug(&format!("AST:\n{}", tree));

    log::debug("Evaluating program...");

    let now = std::time::Instant::now();
    let out = interpreter::evaluate(program).unwrap_or_else(|err| exit_with_info(err, &source));

    if let Some(output) = out {
        println!("{}", output);
    }
    let elapsed = now.elapsed();
    log::info(&format!("Execution time: {}s", elapsed.as_secs_f64()));

    log::info("Exiting");
}
