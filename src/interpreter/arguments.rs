use {
    crate::{
        errors,
        errors::CompilerError,
        grammar::Argument,
        interpreter::{environment::Environment, expressions, functions},
        tokens::Value,
    },
    std::{cell::RefCell, rc::Rc},
};

pub fn eval(env: Rc<RefCell<Environment>>, arg: &Argument) -> Result<Value, CompilerError> {
    match arg {
        Argument::Value(v) => Ok(v.clone()),
        Argument::Identifier(l_id) => {
            let value = env
                .borrow()
                .get(&l_id.id)
                .map(|f| functions::eval(Rc::clone(&env), &f));
            if value.is_none() {
                return Err(errors::undefined_argument(&l_id.id, l_id.line));
            }
            let value = value.unwrap()?;
            Ok(value.unwrap())
        }
        Argument::ParenExpression(expr) => {
            let result = expressions::eval(env, expr)?;
            if let Some(v) = result {
                Ok(v)
            } else {
                Err(CompilerError::Interpreter(
                    "parenthesized expression did not evaluate to a value".to_string(),
                    0,
                ))
            }
        }
    }
}
