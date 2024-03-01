use {
    crate::tokens::{Identifier, Operator, Type},
    std::{
        error::Error,
        fmt::{self, Display, Formatter},
    },
};

#[derive(PartialEq, Debug)]
pub enum ApplicationError {
    Args(String),
}

impl Error for ApplicationError {}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ApplicationError::Args(e) => write!(f, "Invalid program arguments: {}", e),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CompilerError {
    Lexer(String, usize),
    Parser(String, usize),
    Interpreter(String, usize),
}

impl Error for CompilerError {}

impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CompilerError::Lexer(e, line) => {
                write!(f, "Lexer error: {} (line {})", e, line)
            }
            CompilerError::Parser(e, line) => {
                write!(f, "Parser error: {} (line {})", e, line)
            }
            CompilerError::Interpreter(e, line) => {
                if line > &0 {
                    write!(f, "Interpreter error: {} (line {})", e, line)
                } else {
                    write!(f, "Interpreter error: {}", e)
                }
            }
        }
    }
}

pub fn unexpected_type(expected: &Type, observed: &Type, line: usize) -> CompilerError {
    CompilerError::Interpreter(format!("expected {}, found {}", expected, observed), line)
}

pub fn unexpected_type_class(expected: &str, observed: &Type, line: usize) -> CompilerError {
    CompilerError::Interpreter(format!("expected {}, found {}", expected, observed), line)
}

pub fn undefined_variable(identifier: &Identifier, line: usize) -> CompilerError {
    CompilerError::Interpreter(format!("undefined variable `{}`", identifier), line)
}

pub fn undefined_argument(identifier: &Identifier, line: usize) -> CompilerError {
    CompilerError::Interpreter(format!("no argument passed for `{}`", identifier), line)
}

pub fn unexpected_token(token: &str, line: usize) -> CompilerError {
    CompilerError::Interpreter(format!("expected `{}`", token), line)
}

pub fn wrong_operator_arity(operator: &Operator, line: usize) -> CompilerError {
    CompilerError::Interpreter(
        format!(
            "operator `{}` expects {}",
            operator,
            operator.operator_type().arity()
        ),
        line,
    )
}
