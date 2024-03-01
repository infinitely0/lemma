use {
    crate::{
        errors,
        errors::CompilerError,
        grammar::{Call, Conditional, Expression},
        interpreter::{calls, data, environment::Environment, operations},
        tokens::{Type, Value},
    },
    std::{cell::RefCell, rc::Rc},
};

pub fn eval(
    env: Rc<RefCell<Environment>>,
    expr: &Expression,
) -> Result<Option<Value>, CompilerError> {
    match expr {
        Expression::Value(v) => {
            if let Value::List(l) = v {
                data::eval_list(env, l)
            } else {
                Ok(Some(v.clone()))
            }
        }
        Expression::Call(c) => match c {
            Call::Operation(op) => operations::eval(env, op),
            Call::FunctionCall(fc) => calls::eval(env, fc),
            Call::Pipe(p) => calls::eval_pipe(env, p),
        },
        Expression::Conditional(c) => eval_conditional(env, c),
    }
}

fn eval_conditional(
    env: Rc<RefCell<Environment>>,
    cdl: &Conditional,
) -> Result<Option<Value>, CompilerError> {
    match eval(Rc::clone(&env), &cdl.condition)? {
        Some(Value::Boolean(b)) => {
            if b {
                Ok(eval(env, &cdl.then)?)
            } else {
                Ok(eval(env, &cdl.otherwise)?)
            }
        }
        Some(v) => Err(errors::unexpected_type(
            &Type::Bool,
            &v.get_type(),
            cdl.line,
        )),
        None => Ok(None),
    }
}
