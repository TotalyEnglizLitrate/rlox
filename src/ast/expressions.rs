use crate::tokens::{Literal, Operator};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Operator,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn interpret(&self) {}
    pub fn resolve(&self) {}
    pub fn analyze(&self) {}
}
