use super::consume;
use super::Scanner;

use crate::ast::stmt::Stmt;
use crate::error::{Error, ErrorKind};
use crate::tokens::Operator;
use crate::tokens::{Keyword, Literal, Punctuator, Token};

impl Scanner {
    pub fn statement(&mut self) -> Option<Stmt> {
        let tok = self.peek().token;

        if matches!(tok, Token::Keyword(Keyword::PRINT)) {
            self.print()
        } else if matches!(tok, Token::Keyword(Keyword::VAR)) {
            self.decl()
        } else {
            self.expr_stmt()
        }
    }

    fn print(&mut self) -> Option<Stmt> {
        self.advance();
        let expr = self.expression();
        if consume!(self, Token::Punctuator(Punctuator::SEMICOLON)) {
            Some(Stmt::Print(*expr))
        } else {
            None
        }
    }

    fn expr_stmt(&mut self) -> Option<Stmt> {
        let expr = self.expression();
        if consume!(self, Token::Punctuator(Punctuator::SEMICOLON)) {
            Some(Stmt::Expr(*expr))
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
                let expr = self.expression();
                Some(*expr)
            }
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

        if consume!(self, Token::Punctuator(Punctuator::SEMICOLON)) {
            Some(Stmt::Declaration(Literal::IDENT(name), expr))
        } else {
            None
        }
    }
}
