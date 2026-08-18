use std::io::{Write, stdout};

use crate::{ast::expr::Expr, error::Error};

type Block = Vec<Box<Stmt>>;

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Assignment(Expr),
    Expr(Expr),
    If(Expr, Block),
}

impl Stmt {
    pub(crate) fn parse(self) -> Result<(), Error> {
        match self {
            Stmt::Print(expr) => {
                match expr.evaluate() {
                    Ok(val) => {
                        println!("{}", val);
                        Ok(())
                    },
                    Err(err) => Err(err)
                }
            },
            Stmt::Expr(expr) => {
                if let Err(err) = expr.evaluate() {
                    Err(err)
                } else {
                    Ok(())
                }
            },
            Stmt::Assignment(_) => unimplemented!(),
            Stmt::If(_, _) => unimplemented!(),
        }
    }
}
