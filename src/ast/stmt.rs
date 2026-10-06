use crate::{ast::expr::Expr, environment::Environment, error::Error, tokens::Literal};

type Block = Vec<Box<Stmt>>;

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Declaration(Literal, Option<Expr>),
    Expr(Expr),
}

impl Stmt {
    pub(crate) fn parse(self, env: &mut Environment) -> Result<(), Error> {
        match self {
            Stmt::Print(expr) => match expr.evaluate(env) {
                Ok(val) => {
                    println!("{}", val);
                    Ok(())
                }
                Err(err) => Err(err),
            },
            Stmt::Expr(expr) => {
                if let Err(err) = expr.evaluate(env) {
                    Err(err)
                } else {
                    Ok(())
                }
            }
            Stmt::Declaration(name, expr) => {
                if let Literal::IDENT(name) = name {
                    match expr
                        .unwrap_or(Expr::Literal {
                            value: Literal::NIL,
                        })
                        .evaluate(env)
                    {
                        Ok(val) => {
                            env.define(name, val);
                            Ok(())
                        }
                        Err(err) => Err(err),
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }
}
