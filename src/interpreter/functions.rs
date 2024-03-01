use {
    crate::{
        errors::CompilerError,
        grammar::{Definition, Function, Signature},
        interpreter::{expressions, Environment},
        tokens::Value,
    },
    std::{cell::RefCell, rc::Rc},
};

pub fn eval(
    env: Rc<RefCell<Environment>>,
    function: &Function,
) -> Result<Option<Value>, CompilerError> {
    let signature = &function.signature;
    let definition = &function.definition;
    validate_arity(signature, definition)?;

    let expr = &definition.body;
    expressions::eval(env, expr)
}

fn validate_arity(signature: &Signature, definition: &Definition) -> Result<(), CompilerError> {
    if signature.parameters.len() != definition.parameters.len() {
        let error = format!(
            "function signature and definition arity mismatch: expected {} parameters, found {}",
            signature.parameters.len(),
            definition.parameters.len()
        );

        return Err(CompilerError::Interpreter(error, definition.name.line));
    }
    Ok(())
}
