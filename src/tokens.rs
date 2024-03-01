use std::fmt::{self, Display, Formatter};

use {
    crate::grammar::{Element, List},
    serde::{Deserialize, Serialize},
    Operator::{Add, And, Div, Eq, Gt, Gte, Lt, Lte, Mul, Neq, Not, Or, Sub},
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Token {
    Identifier(Identifier, usize),
    Symbol(Symbol, usize),
    Operator(Operator, usize),
    Type(Type, usize),
    Value(Value, usize),
    Conditional(Conditional, usize),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocatedIdentifier {
    pub id: Identifier,
    pub line: usize,
}

pub type Identifier = String;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Symbol {
    LP,
    RP,
    LB,
    RB,
    Comment,
    Assign,
    Return,
    Pipe,
    Bar,
    Range,
    EOL,
    EOF,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Type {
    Int,
    Frac,
    Str,
    Bool,
    Void,
    List(Box<Type>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    Or,
    And,
    Not,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum OperatorType {
    Unary,
    Binary,
    Relational,
    Logical,
}

impl OperatorType {
    pub fn arity(&self) -> String {
        match self {
            OperatorType::Unary => "exactly 1 argument".into(),
            OperatorType::Binary => "at least 2 argument".into(),
            OperatorType::Relational => "exactly 2 arguments".into(),
            OperatorType::Logical => "exactly 2 arguments".into(),
        }
    }
}

impl Operator {
    pub fn operator_type(&self) -> OperatorType {
        match self {
            Add | Sub | Mul | Div => OperatorType::Binary,
            Gt | Lt | Gte | Lte => OperatorType::Relational,
            Eq | Neq | And | Or => OperatorType::Logical,
            Not => OperatorType::Unary,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Value {
    Integer(i64),
    Fractional(f64),
    String(String),
    Boolean(bool),
    List(List),
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::Integer(_) => Type::Int,
            Value::Fractional(_) => Type::Frac,
            Value::String(_) => Type::Str,
            Value::Boolean(_) => Type::Bool,
            Value::List(list) => Type::List(Box::new(list.list_type.clone())),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Conditional {
    If,
    Then,
    Else,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl From<Token> for String {
    fn from(token: Token) -> Self {
        match token {
            Token::Identifier(t, _line) => t,
            Token::Symbol(t, _line) => t.into(),
            Token::Operator(t, _line) => t.into(),
            Token::Type(t, _line) => t.into(),
            Token::Value(t, _line) => t.into(),
            Token::Conditional(t, _line) => t.into(),
        }
    }
}

impl From<Symbol> for String {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::LP => "(".to_string(),
            Symbol::RP => ")".to_string(),
            Symbol::LB => "[".to_string(),
            Symbol::RB => "]".to_string(),
            Symbol::Comment => "#".to_string(),
            Symbol::Assign => "=".to_string(),
            Symbol::Return => "->".to_string(),
            Symbol::Pipe => ">>".to_string(),
            Symbol::Bar => "|".to_string(),
            Symbol::Range => "..".to_string(),
            Symbol::EOL => "\n".to_string(),
            Symbol::EOF => "".to_string(),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl From<Operator> for String {
    fn from(operator: Operator) -> Self {
        match operator {
            Add => "+".to_string(),
            Sub => "-".to_string(),
            Mul => "*".to_string(),
            Div => "/".to_string(),
            Eq => "==".to_string(),
            Neq => "!=".to_string(),
            Gt => ">".to_string(),
            Lt => "<".to_string(),
            Gte => ">=".to_string(),
            Lte => "<=".to_string(),
            Or => "||".to_string(),
            And => "&&".to_string(),
            Not => "!".to_string(),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl From<Type> for String {
    fn from(t: Type) -> Self {
        match t {
            Type::Int => "Int".to_string(),
            Type::Frac => "Frac".to_string(),
            Type::Str => "Str".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::Void => "Void".to_string(),
            Type::List(t) => format!("[{}]", t),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        match value {
            Value::Integer(t) => t.to_string(),
            Value::Fractional(t) => {
                let s = t.to_string();
                if !s.contains('.') {
                    format!("{}.0", s)
                } else {
                    s
                }
            }
            Value::String(t) => t.to_string(),
            Value::Boolean(t) => t.to_string(),
            Value::List(t) => t.to_string(),
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s = "[".to_string();
        for (i, element) in self.elements.iter().enumerate() {
            s.push_str(&element.to_string());
            if i < self.elements.len() - 1 {
                s.push_str(", ");
            }
        }
        s.push(']');
        write!(f, "{}", s)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Element::Value(t) => write!(f, "{}", t),
            Element::Identifier(t) => write!(f, "{}", t),
        }
    }
}

impl From<Conditional> for String {
    fn from(cond: Conditional) -> Self {
        match cond {
            Conditional::If => "if".to_string(),
            Conditional::Then => "then".to_string(),
            Conditional::Else => "else".to_string(),
        }
    }
}

impl Display for Conditional {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}
