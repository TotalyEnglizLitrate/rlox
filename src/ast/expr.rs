use std::cmp::Ordering;

use crate::{
    error::{Error, ErrorKind},
    tokens::{Literal, Operator},
};

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Operator,
        line: usize,
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
        line: usize,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn evaluate(self) -> Result<Literal, Error> {
        match self {
            Expr::Grouping { expression } => expression.evaluate(),
            Expr::Unary {
                operator,
                line,
                right,
            } => Self::eval_unary(&operator, &right.evaluate()?, line),
            Expr::Binary {
                left,
                operator,
                line,
                right,
            } => Self::eval_binary(&left.evaluate()?, &right.evaluate()?, &operator, line),
            Expr::Literal { value } => Ok(value),
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
        left: &Literal,
        right: &Literal,
        op: &Operator,
        line: usize,
    ) -> Result<Literal, Error> {
        match op {
            Operator::MINUS => (left - right).ok_or_else(|| Expr::op_error(left, right, op, line)),
            Operator::PLUS => (left + right).ok_or_else(|| Expr::op_error(left, right, op, line)),
            Operator::STAR => (left * right).ok_or_else(|| Expr::op_error(left, right, op, line)),
            Operator::SLASH => (left / right).ok_or_else(|| Expr::op_error(left, right, op, line)),
            Operator::EQ => Ok((left == right).into()),
            Operator::NE => Ok((left != right).into()),
            Operator::GREATER => left
                .partial_cmp(right)
                .ok_or_else(|| Expr::op_error(left, right, op, line))
                .map(|ord| (ord == Ordering::Greater).into()),
            Operator::LESSER => left
                .partial_cmp(right)
                .ok_or_else(|| Expr::op_error(left, right, op, line))
                .map(|ord| (ord == Ordering::Less).into()),
            Operator::GE => left
                .partial_cmp(right)
                .ok_or_else(|| Expr::op_error(left, right, op, line))
                .map(|ord| matches!(ord, Ordering::Greater | Ordering::Equal).into()),
            Operator::LE => left
                .partial_cmp(right)
                .ok_or_else(|| Expr::op_error(left, right, op, line))
                .map(|ord| matches!(ord, Ordering::Less | Ordering::Equal).into()),
            _ => todo!(),
        }
    }
}
