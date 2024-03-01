use {
    crate::{grammar::Function, tokens::Identifier},
    std::{cell::RefCell, collections::HashMap, rc::Rc},
};

pub struct Environment {
    scope: HashMap<Identifier, Function>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(scope: HashMap<Identifier, Function>) -> Self {
        Self {
            scope,
            enclosing: None,
        }
    }

    pub fn with_enclosing(
        scope: HashMap<Identifier, Function>,
        enclosing: Rc<RefCell<Environment>>,
    ) -> Self {
        Self {
            scope,
            enclosing: Some(enclosing),
        }
    }

    pub fn get(&self, id: &Identifier) -> Option<Function> {
        match self.scope.get(id) {
            Some(f) => Some(f.clone()),
            None => match &self.enclosing {
                Some(env) => env.borrow().get(id),
                None => None,
            },
        }
    }
}
