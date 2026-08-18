pub mod expr;
pub mod stmt;

use crate::ast::stmt::Stmt;
use crate::error::Error;
use crate::tokens::{Token, TokenCtx};

macro_rules! multi_matches {
    ($expr:expr, $($pattern:pat_param)|+ $(if $guard:expr)?) => {
        matches!($expr, $($pattern)|+ $(if $guard)?)
    };
}

macro_rules! consume {
    ($parser:expr, $expected:pat) => {{
        let ctx = $parser.peek();

        if matches!(ctx.token, $expected) {
            $parser.advance();
            true
        } else {
            $parser.errors.push(Error::new(
                ctx.line,
                "".into(),
                format!(
                    "Expected {}, found {:?}",
                    stringify!($expected),
                    ctx.token
                ),
                ErrorKind::SyntaxError,
            ));
            false
        }
    }};
}
pub(crate) use multi_matches;
pub(crate) use consume;

pub struct Scanner {
    tokens: Box<[TokenCtx]>,
    pos: usize,
    errors: Vec<Error>,
}

impl Scanner {
    pub fn new(tokens: Box<[TokenCtx]>) -> Self {
        Scanner {
            tokens,
            pos: 0,
            errors: Vec::new(),
        }
    }
    pub fn parse(mut self) -> Result<Vec<Stmt>, Vec<Error>> {
        let mut statements = Vec::new();

        while !matches!(self.peek().token, Token::EOF) {
            match self.statement() {
                Some(stmt) => statements.push(stmt),
                None => break,
            }
        }

        if self.errors.is_empty() {
            Ok(statements)
        } else {
            Err(self.errors)
        }
    }

    fn peek(&self) -> TokenCtx {
        self.tokens[self.pos].clone()
    }

    fn advance(&mut self) {
        if matches!(self.peek().token, Token::EOF) {
            return;
        }
        self.pos += 1;
    }
}
