use serde::{Deserialize, Serialize};

use crate::tokens::{Identifier, LocatedIdentifier, Operator, Type, Value};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
    pub signature: Signature,
    pub definition: Definition,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature {
    pub parameters: Vec<Type>,
    pub returns: Type,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Definition {
    pub name: LocatedIdentifier,
    pub parameters: Vec<LocatedIdentifier>,
    pub body: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Expression {
    Value(Value),
    Call(Call),
    Conditional(Conditional),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Call {
    Operation(Operation),
    FunctionCall(FunctionCall),
    Pipe(Pipe),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Operation {
    pub operator: Operator,
    pub arguments: Vec<Argument>,
    pub line: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    pub name: LocatedIdentifier,
    pub arguments: Vec<Argument>,
    pub line: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pipe {
    pub left: Box<Expression>,
    pub right: Identifier,
    pub line: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conditional {
    pub condition: Box<Expression>,
    pub then: Box<Expression>,
    pub otherwise: Box<Expression>,
    pub line: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Argument {
    Value(Value),
    Identifier(LocatedIdentifier),
    ParenExpression(Expression),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct List {
    pub list_type: Type,
    pub elements: Vec<Element>,
    pub line: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Element {
    Value(Value),
    Identifier(Identifier),
}
