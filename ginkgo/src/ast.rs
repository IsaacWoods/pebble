use crate::interpreter::Value;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Stmt {
    Expression(Expr),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expression(expr) => writeln!(f, "({})", expr),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    Identifier(String),
    UnaryOp { op: UnaryOp, operand: Box<Expr> },
    BinaryOp { op: BinaryOp, left: Box<Expr>, right: Box<Expr> },
    Grouping { inner: Box<Expr> },
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UnaryOp {
    Negate,
    Plus,
    Not,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(value) => match value {
                Value::Integer(value) => write!(f, "{}", value),
                Value::Bool(value) => write!(f, "{}", value),
                Value::String(value) => write!(f, "{}", value),
            },
            Self::Identifier(name) => write!(f, "{}", name),
            Self::UnaryOp { op, operand } => match op {
                UnaryOp::Negate => write!(f, "(- {})", operand),
                UnaryOp::Plus => write!(f, "(+ {})", operand),
                UnaryOp::Not => write!(f, "(! {})", operand),
            },
            Self::BinaryOp { op, left, right } => match op {
                BinaryOp::Add => write!(f, "(+ {} {})", left, right),
                BinaryOp::Subtract => write!(f, "(- {} {})", left, right),
                BinaryOp::Multiply => write!(f, "(* {} {})", left, right),
                BinaryOp::Divide => write!(f, "(/ {} {})", left, right),
            },
            Self::Grouping { inner } => write!(f, "'(' {} ')'", inner),
        }
    }
}
