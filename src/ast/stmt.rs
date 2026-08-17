use crate::ast::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Print(Box<Expr>),
    Assignment(Box<Expr>),
    If {
        cond: Box<Expr>,
        block: Vec<Box<Expr>>,
    },
}

#[derive(Debug)]
enum Conditional {
    cond,
}
