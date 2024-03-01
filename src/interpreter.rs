use {
    crate::{
        errors::CompilerError,
        grammar::{Function, Program},
        interpreter::environment::Environment,
        tokens::{Identifier, Value},
    },
    std::{
        cell::RefCell,
        collections::{hash_map::Entry, HashMap},
        rc::Rc,
    },
};

mod arguments;
mod calls;
mod data;
mod environment;
mod expressions;
mod functions;
mod operations;
mod prelude;

pub fn evaluate(program: Program) -> Result<Option<Value>, CompilerError> {
    let enclosing = Environment::new(prelude::functions());
    let scope: HashMap<Identifier, Function> =
        program
            .functions
            .into_iter()
            .try_fold(HashMap::new(), |mut acc, f| {
                let key = f.definition.name.clone().id;
                match acc.entry(key.clone()) {
                    Entry::Vacant(e) => {
                        e.insert(f);
                        Ok(acc)
                    }
                    Entry::Occupied(_) => Err(CompilerError::Interpreter(
                        format!("variable `{}` already defined", key),
                        f.definition.name.line,
                    )),
                }
            })?;

    let env = Environment::with_enclosing(scope, Rc::new(RefCell::new(enclosing)));

    let main = env
        .get(&"main".to_string())
        .ok_or_else(|| CompilerError::Interpreter("main function not found".to_string(), 0))?;

    functions::eval(Rc::new(RefCell::new(env)), &main)
}
