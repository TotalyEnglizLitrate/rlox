use crate::{ast::expr::Expr, error::Error, tokens::Literal};

type Block = Vec<Box<Stmt>>;

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Declaration(Literal, Option<Expr>),
    Expr(Expr),
}

impl Stmt {
    pub(crate) fn parse(self) -> Result<(), Error> {
        match self {
            Stmt::Print(expr) => match expr.evaluate() {
                Ok(val) => {
                    println!("{}", val);
                    Ok(())
                }
                Err(err) => Err(err),
            },
            Stmt::Expr(expr) => {
                if let Err(err) = expr.evaluate() {
                    Err(err)
                } else {
                    Ok(())
                }
            },
            Stmt::Declaration(_, _) => {
                unimplemented!()
            }
        }
    }
}
