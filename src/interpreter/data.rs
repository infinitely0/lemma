use {
    crate::{
        errors::{undefined_argument, undefined_variable, CompilerError},
        grammar::{Element, List},
        interpreter::{environment::Environment, functions},
        tokens::Value,
    },
    std::{cell::RefCell, rc::Rc},
};

pub fn eval_list(env: Rc<RefCell<Environment>>, l: &List) -> Result<Option<Value>, CompilerError> {
    let list_type = l.list_type.clone();
    let mut resolved: Vec<Element> = Vec::new();
    for e in l.elements.iter() {
        match e {
            Element::Value(v) => {
                if v.get_type() != list_type {
                    return Err(CompilerError::Interpreter(
                        format!(
                            "list type mismatch: expected {}, found {}",
                            list_type,
                            v.get_type()
                        ),
                        l.line,
                    ));
                }
                resolved.push(Element::Value(v.clone()));
            }
            Element::Identifier(i) => {
                let value = env
                    .borrow()
                    .get(i)
                    .map(|f| functions::eval(Rc::clone(&env), &f))
                    .ok_or_else(|| undefined_variable(i, l.line))??
                    .ok_or_else(|| undefined_argument(i, l.line))?;

                if value.get_type() != list_type {
                    return Err(CompilerError::Interpreter(
                        format!(
                            "list type mismatch: expected {}, found {}",
                            list_type,
                            value.get_type()
                        ),
                        l.line,
                    ));
                }

                resolved.push(Element::Value(value));
            }
        }
    }

    let list = List {
        list_type,
        elements: resolved,
        line: l.line,
    };
    Ok(Some(Value::List(list)))
}
