use super::Scanner;
use super::consume;

use crate::ast::stmt::Stmt;
use crate::error::{Error, ErrorKind};
use crate::tokens::Operator;
use crate::tokens::{Keyword, Punctuator, Token, Literal};

impl Scanner {
    pub fn statement(&mut self) -> Option<Stmt> {
        let tok = self.peek().token;

        if matches!(tok, Token::Keyword(Keyword::PRINT)) {
            self.print()
        } else if matches!(tok, Token::Keyword(Keyword::VAR)) {
            self.decl()
        } else {
            self.expr()
        }
    }

    fn print(&mut self) -> Option<Stmt> {
        self.advance();
        if let Some(Stmt::Expr(expr)) = self.expr() {
            Some(Stmt::Print(expr))
        } else {
            None
        }
    }

    fn expr(&mut self)  -> Option<Stmt> {
        let stmt = Stmt::Expr(*self.expression());
        if consume!(self, Token::Punctuator(Punctuator::SEMICOLON)) {
            Some(stmt)
        } else {
            None
        }

    }

    fn decl(&mut self) -> Option<Stmt> {
        consume!(self, Token::Keyword(Keyword::VAR));
        let name = match self.peek().token {
            Token::Literal(Literal::IDENT(name)) => name,
            token => {
                self.errors.push(Error::new(
                    self.peek().line,
                    "".into(),
                    format!("Expected identifier, found {:?}", token),
                    ErrorKind::SyntaxError,
                ));
                return None;
            }
        };

        self.advance();

        let expr = match self.peek().token {
            Token::Punctuator(Punctuator::SEMICOLON) => None,
            Token::Operator(Operator::ASSIGN) => {
                self.advance();
                if let Some(Stmt::Expr(expr)) = self.expr() {
                    Some(expr)
                } else {
                    None
                }
            },
            token => {
                self.errors.push(Error::new(
                    self.peek().line,
                    "".into(),
                    format!("Expected ';' or '=', found {:?}", token),
                    ErrorKind::SyntaxError,
                ));
                return None;
            }
        };

        self.advance();

        Some(Stmt::Declaration(Literal::IDENT(name), expr))
    }
}
