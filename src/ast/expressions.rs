use std::ops::Deref;

use crate::{
    error::{Error, ErrorKind},
    tokens::{Literal, Operator, Token, TokenCtx},
};

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Box<TokenCtx>,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Box<TokenCtx>,
    },
    Unary {
        operator: Box<TokenCtx>,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn validate(&self) -> bool {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                matches!(operator.deref().token, Token::Operator(_))
                    && left.validate()
                    && right.validate()
            }
            Expr::Grouping { expression } => expression.validate(),
            Expr::Literal { value } => matches!(value.deref().token, Token::Literal(_)),
            Expr::Unary { operator, right } => {
                matches!(operator.deref().token, Token::Operator(_)) && right.validate()
            }
        }
    }

    pub fn evaluate(self) -> Result<Literal, Error> {
        (!self.validate()).then(|| panic!("Error in constructed ast"));

        match self {
            Expr::Grouping { expression } => expression.evaluate(),
            Expr::Unary { operator, right } => {
                if let Token::Operator(op) = operator.token {
                    Self::eval_unary(&op, &right.evaluate()?, operator.line)
                } else {
                    unreachable!()
                }
            }
            _ => todo!(),
        }
    }

    fn eval_unary(op: &Operator, right: &Literal, line: usize) -> Result<Literal, Error> {
        match op {
            Operator::MINUS => {
                if let Literal::NUMBER(num) = right {
                    Ok(Literal::NUMBER(-num))
                } else {
                    Err(Error::new(
                        line,
                        "".into(),
                        format!("Invalid operand {:?} for operation {:?}", right, op),
                        ErrorKind::TypeError,
                    ))
                }
            }

            Operator::BANG => Ok(!right),
            _ => unreachable!(),
        }
    }

    fn op_error(right: &Literal, left: &Literal, op: &Operator, line: usize) -> Error {
        Error::new(
            line,
            "".into(),
            format!(
                "Unsupported operation {:?} between types {} and {}",
                op,
                left.get_type(),
                right.get_type()
            )
            .into(),
            ErrorKind::TypeError,
        )
    }

    fn eval_binary(
        right: &Literal,
        left: &Literal,
        op: &Operator,
        line: usize,
    ) -> Result<Literal, Error> {
        match op {
            Operator::MINUS => (left - right).ok_or(Expr::op_error(right, left, op, line)),
            Operator::PLUS => (left + right).ok_or(Expr::op_error(right, left, op, line)),
            Operator::STAR => (left * right).ok_or(Expr::op_error(right, left, op, line)),
            Operator::SLASH => (left / right).ok_or(Expr::op_error(right, left, op, line)),
            Operator::EQ => Ok((left == right).into()),
            Operator::NE => Ok((left != right).into()),
            _ => todo!(),
        }
    }
}
