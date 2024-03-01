use {
    crate::{
        errors,
        errors::CompilerError,
        grammar::Operation,
        interpreter::{arguments, Environment},
        tokens::{
            Operator, OperatorType,
            Type::{self, Bool},
            Value,
        },
    },
    std::{cell::RefCell, rc::Rc},
};

pub fn eval(
    env: Rc<RefCell<Environment>>,
    operation: &Operation,
) -> Result<Option<Value>, CompilerError> {
    match operation.operator.operator_type() {
        OperatorType::Unary => eval_unary(env, operation),
        OperatorType::Binary => eval_binary(env, operation),
        OperatorType::Relational => eval_relational(env, operation),
        OperatorType::Logical => eval_logical(env, operation),
    }
}

fn eval_unary(
    env: Rc<RefCell<Environment>>,
    op: &Operation,
) -> Result<Option<Value>, CompilerError> {
    if op.arguments.len() != 1 {
        return Err(errors::wrong_operator_arity(&op.operator, op.line));
    }

    // Unary operations for numeric values are handled as binary operations
    if op.operator != Operator::Not {
        panic!("invalid unary operator");
    }

    let value = arguments::eval(env, op.arguments.first().unwrap())?;
    match value {
        Value::Boolean(b) => Ok(Some(Value::Boolean(!b))),
        _ => Err(errors::unexpected_type(&Bool, &value.get_type(), op.line)),
    }
}

fn eval_binary(
    env: Rc<RefCell<Environment>>,
    op: &Operation,
) -> Result<Option<Value>, CompilerError> {
    let line = op.line;

    // Unary operations handled here for numeric values. -x is treated as 0 - x. +x is treated as x.
    if op.arguments.len() == 1 {
        return if let Operator::Add | Operator::Sub = op.operator {
            eval_unary_numeric(Rc::clone(&env), op)
        } else {
            Err(errors::wrong_operator_arity(&op.operator, op.line))
        };
    }

    if op.arguments.len() < 2 {
        return Err(errors::wrong_operator_arity(&op.operator, op.line));
    }

    let value = arguments::eval(Rc::clone(&env), op.arguments.first().unwrap())?;
    match value {
        Value::Integer(i) => Ok(eval_int_op(Rc::clone(&env), op, i)?),
        Value::Fractional(f) => Ok(eval_frac_op(Rc::clone(&env), op, f)?),
        _ => Err(errors::unexpected_type_class(
            "Numeric",
            &value.get_type(),
            line,
        )),
    }
}

fn eval_relational(
    env: Rc<RefCell<Environment>>,
    op: &Operation,
) -> Result<Option<Value>, CompilerError> {
    if op.arguments.len() != 2 {
        return Err(errors::wrong_operator_arity(&op.operator, op.line));
    }

    let first = arguments::eval(Rc::clone(&env), op.arguments.first().unwrap())?;
    let second = arguments::eval(Rc::clone(&env), op.arguments.get(1).unwrap())?;

    match (&first, &second) {
        (Value::Integer(i), Value::Integer(j)) => {
            let result = match op.operator {
                Operator::Lt => i < j,
                Operator::Lte => i <= j,
                Operator::Gt => i > j,
                Operator::Gte => i >= j,
                _ => panic!("not a relational operator"),
            };
            Ok(Some(Value::Boolean(result)))
        }
        (Value::Fractional(f), Value::Fractional(g)) => {
            let result = match op.operator {
                Operator::Lt => f < g,
                Operator::Lte => f <= g,
                Operator::Gt => f > g,
                Operator::Gte => f >= g,
                _ => panic!("not a relational operator"),
            };
            Ok(Some(Value::Boolean(result)))
        }
        _ => Err(errors::unexpected_type(
            &first.get_type(),
            &second.get_type(),
            op.line,
        )),
    }
}

fn eval_logical(
    env: Rc<RefCell<Environment>>,
    op: &Operation,
) -> Result<Option<Value>, CompilerError> {
    if op.arguments.len() != 2 {
        return Err(errors::wrong_operator_arity(&op.operator, op.line));
    }

    let first = arguments::eval(Rc::clone(&env), op.arguments.first().unwrap())?;
    let second = arguments::eval(Rc::clone(&env), op.arguments.get(1).unwrap())?;

    if let Value::Boolean(_) = first {
        match (first, second.clone()) {
            (Value::Boolean(a), Value::Boolean(b)) => {
                let result = eval_bool_op(&op.operator, a, b);
                Ok(Some(Value::Boolean(result)))
            }
            _ => Err(errors::unexpected_type(&Bool, &second.get_type(), op.line)),
        }
    } else {
        // If the first argument is not a boolean, assume comparison of numeric values
        match (first, second) {
            (Value::Integer(i), Value::Integer(j))
                if op.operator == Operator::Eq || op.operator == Operator::Neq =>
            {
                let result = if op.operator == Operator::Eq {
                    i == j
                } else {
                    i != j
                };
                Ok(Some(Value::Boolean(result)))
            }
            (Value::Fractional(f), Value::Fractional(g))
                if op.operator == Operator::Eq || op.operator == Operator::Neq =>
            {
                let result = if op.operator == Operator::Eq {
                    f == g
                } else {
                    f != g
                };
                Ok(Some(Value::Boolean(result)))
            }
            _ => Err(CompilerError::Interpreter(
                "type mismatch".to_string(),
                op.line,
            )),
        }
    }
}

fn eval_unary_numeric(
    env: Rc<RefCell<Environment>>,
    op: &Operation,
) -> Result<Option<Value>, CompilerError> {
    let line = op.line;
    let arg = op.arguments.first().unwrap();
    let value = arguments::eval(env, arg)?;

    if op.operator == Operator::Sub {
        match value {
            Value::Integer(i) => Ok(Some(Value::Integer(-i))),
            Value::Fractional(f) => Ok(Some(Value::Fractional(-f))),
            _ => Err(errors::unexpected_type_class(
                "Numeric",
                &value.get_type(),
                line,
            )),
        }
    } else {
        match value {
            Value::Integer(i) => Ok(Some(Value::Integer(i))),
            Value::Fractional(f) => Ok(Some(Value::Fractional(f))),
            _ => Err(errors::unexpected_type_class(
                "Numeric",
                &value.get_type(),
                line,
            )),
        }
    }
}

fn eval_int_op(
    env: Rc<RefCell<Environment>>,
    op: &Operation,
    first: i64,
) -> Result<Option<Value>, CompilerError> {
    let mut args: Vec<i64> = Vec::new();
    args.push(first);

    for arg in op.arguments.iter().skip(1) {
        let v = arguments::eval(Rc::clone(&env), arg)?;
        if let Value::Integer(i) = v {
            args.push(i);
        } else {
            return Err(errors::unexpected_type(&Type::Int, &v.get_type(), op.line));
        }
    }

    match eval_arithmetic(&op.operator, &args) {
        Ok(result) => Ok(Some(Value::Integer(result))),
        Err(e) => Err(CompilerError::Interpreter(e, op.line)),
    }
}

fn eval_frac_op(
    env: Rc<RefCell<Environment>>,
    op: &Operation,
    first: f64,
) -> Result<Option<Value>, CompilerError> {
    let mut args: Vec<f64> = Vec::new();
    args.push(first);

    for arg in op.arguments.iter().skip(1) {
        let v = arguments::eval(Rc::clone(&env), arg)?;
        if let Value::Fractional(f) = v {
            args.push(f);
        } else {
            return Err(errors::unexpected_type(&Type::Frac, &v.get_type(), op.line));
        }
    }

    match eval_arithmetic(&op.operator, &args) {
        Ok(result) => Ok(Some(Value::Fractional(result))),
        Err(e) => Err(CompilerError::Interpreter(e, op.line)),
    }
}

fn eval_arithmetic<
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + PartialEq
        + Copy
        + From<i32>,
>(
    operator: &Operator,
    args: &[T],
) -> Result<T, String> {
    match operator {
        Operator::Add => Ok(args.iter().fold(T::from(0), |acc, &arg| acc + arg)),
        Operator::Sub => Ok(args[1..].iter().fold(args[0], |acc, &arg| acc - arg)),
        Operator::Mul => Ok(args.iter().fold(T::from(1), |acc, &arg| acc * arg)),
        Operator::Div => {
            if args[1..].iter().any(|&arg| arg == T::from(0)) {
                return Err("division by zero".to_string());
            }
            Ok(args[1..].iter().fold(args[0], |acc, &arg| acc / arg))
        }
        _ => panic!("not a binary operator"),
    }
}

fn eval_bool_op(operator: &Operator, first: bool, second: bool) -> bool {
    match operator {
        Operator::Eq => first == second,
        Operator::Neq => first != second,
        Operator::And => first && second,
        Operator::Or => first || second,
        _ => panic!("not a boolean operator"),
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::grammar::Argument, std::collections::HashMap};

    #[test]
    fn integer_addition() {
        let op = Operator::Add;
        let args = vec![1, 2, 3, 4, 5];
        let result = eval_arithmetic(&op, &args).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn integer_subtraction() {
        let op = Operator::Sub;
        let args = vec![15, 20, 1];
        let result = eval_arithmetic(&op, &args).unwrap();
        assert_eq!(result, -6);
    }

    #[test]
    fn fractional_multiplication() {
        let op = Operator::Mul;
        let args = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = eval_arithmetic(&op, &args).unwrap();
        assert_eq!(result, 120.0);
    }

    #[test]
    fn fractional_division() {
        let op = Operator::Div;
        let args = vec![15.0, 3.0, 2.0];
        let result = eval_arithmetic(&op, &args).unwrap();
        assert_eq!(result, 2.5);
    }

    #[test]
    fn boolean_eq() {
        let op = Operator::Eq;
        let result = eval_bool_op(&op, true, true);
        assert!(result);
    }

    #[test]
    fn boolean_neq() {
        let op = Operator::Neq;
        let result = eval_bool_op(&op, true, false);
        assert!(result);
    }

    #[test]
    fn boolean_and() {
        let op = Operator::And;
        let result = eval_bool_op(&op, true, false);
        assert!(!result);
    }

    #[test]
    fn boolean_or() {
        let op = Operator::Or;
        let result = eval_bool_op(&op, true, false);
        assert!(result);
    }

    #[test]
    fn eval_logical_op() {
        let env = Rc::new(RefCell::new(Environment::new(HashMap::new())));

        let ops = vec![
            (Operator::Eq, true, true, true),
            (Operator::Neq, true, false, true),
            (Operator::Or, true, false, true),
            (Operator::And, true, false, false),
        ];

        for (op, first, second, expected) in ops {
            let operation = Operation {
                operator: op,
                arguments: vec![
                    Argument::Value(Value::Boolean(first)),
                    Argument::Value(Value::Boolean(second)),
                ],
                line: 0,
            };

            let result = eval_logical(Rc::clone(&env), &operation).unwrap().unwrap();
            assert_eq!(result, Value::Boolean(expected));
        }
    }
}
