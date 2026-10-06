use crate::{ast::expr::Expr, environment::Environment, error::Error, tokens::Literal};


#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Declaration(Literal, Option<Expr>),
    Expr(Expr),
}

impl Stmt {
    pub(crate) fn parse(self, env: &mut Environment, interpreted: bool) -> Result<(), Error> {
        match self {
            Stmt::Print(expr) => match expr.evaluate(env) {
                Ok(val) => {
                    println!("{}", val);
                    Ok(())
                }
                Err(err) => Err(err),
            },
            Stmt::Expr(expr) => match expr.evaluate(env) {
                Ok(val) => {
                    if interpreted {
                        println!("{}", val);
                    }
                    Ok(())
                }
                Err(err) => Err(err),
            },
            Stmt::Declaration(name, expr) => {
                if let Literal::IDENT(var) = name {
                    let val = match expr {
                        Some(e) => e.evaluate(env)?,
                        None => Literal::NIL,
                    };
                    env.define(var, val);
                    Ok(())
                } else {
                    unreachable!()
                }
            }
        }
    }
}
