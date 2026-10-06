use super::multi_matches;
use super::Scanner;

use crate::ast::expr::Expr;
use crate::error::{Error, ErrorKind};
use crate::tokens::{Keyword, Literal, Operator, Punctuator, Token, TokenCtx};

impl Scanner {
    pub fn expression(&mut self) -> Box<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Box<Expr> {
        let expr = self.equality();

        if matches!(self.peek().token, Token::Operator(Operator::ASSIGN)) {
            let tok = self.peek();
            self.advance();
            let value = self.assignment();

            if let Expr::Literal {
                value: Literal::IDENT(name),
                line: _,
            } = *expr
            {
                return Box::new(Expr::Assign {
                    name: Literal::IDENT(name),
                    line: tok.line,
                    value,
                });
            }

            self.errors.push(Error::new(
                tok.line,
                "".into(),
                "Invalid assignment target.".into(),
                ErrorKind::SyntaxError,
            ));
        }

        expr
    }

    fn equality(&mut self) -> Box<Expr> {
        let mut expr = self.comparison();
        let mut tok = self.peek();

        while multi_matches!(
            tok.token,
            Token::Operator(Operator::NE) | Token::Operator(Operator::EQ)
        ) {
            self.advance();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: match tok.token {
                    Token::Operator(op) => op,
                    _ => unreachable!(),
                },
                line: tok.line,
                right: self.comparison(),
            });
            tok = self.peek();
        }

        expr
    }

    fn comparison(&mut self) -> Box<Expr> {
        let mut expr = self.term();
        let mut tok = self.peek();

        while multi_matches!(
            tok.token,
            Token::Operator(Operator::GREATER)
                | Token::Operator(Operator::GE)
                | Token::Operator(Operator::LESSER)
                | Token::Operator(Operator::LE)
        ) {
            self.advance();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: match tok.token {
                    Token::Operator(op) => op,
                    _ => unreachable!(),
                },
                line: tok.line,
                right: self.term(),
            });
            tok = self.peek();
        }

        expr
    }

    fn term(&mut self) -> Box<Expr> {
        let mut expr = self.factor();
        let mut tok = self.peek();

        while multi_matches!(
            tok.token,
            Token::Operator(Operator::MINUS) | Token::Operator(Operator::PLUS)
        ) {
            self.advance();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: match tok.token {
                    Token::Operator(op) => op,
                    _ => unreachable!(),
                },
                line: tok.line,
                right: self.factor(),
            });
            tok = self.peek();
        }

        expr
    }

    fn factor(&mut self) -> Box<Expr> {
        let mut expr = self.unary();
        let mut tok = self.peek();

        while multi_matches!(
            tok.token,
            Token::Operator(Operator::SLASH) | Token::Operator(Operator::STAR)
        ) {
            self.advance();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: match tok.token {
                    Token::Operator(op) => op,
                    _ => unreachable!(),
                },
                line: tok.line,
                right: self.unary(),
            });
            tok = self.peek();
        }

        expr
    }

    fn unary(&mut self) -> Box<Expr> {
        let tok = self.peek();
        if multi_matches!(
            tok.token,
            Token::Operator(Operator::BANG) | Token::Operator(Operator::MINUS)
        ) {
            self.advance();
            return Box::new(Expr::Unary {
                operator: match tok.token {
                    Token::Operator(op) => op,
                    _ => unreachable!(),
                },
                line: tok.line,
                right: self.primary(),
            });
        }

        return self.primary();
    }

    fn primary(&mut self) -> Box<Expr> {
        let tok = self.peek();
        if multi_matches!(
            tok.token,
            Token::Literal(Literal::TRUE)
                | Token::Literal(Literal::FALSE)
                | Token::Literal(Literal::NIL)
        ) {
            self.advance();
            Box::new(Expr::Literal {
                value: match tok.token {
                    Token::Literal(value) => value,
                    _ => unreachable!(),
                },
                line: tok.line,
            })
        } else if matches!(tok.token, Token::Literal(_)) {
            self.advance();
            Box::new(Expr::Literal {
                value: match tok.token {
                    Token::Literal(value) => value,
                    _ => unreachable!(),
                },
                line: tok.line,
            })
        } else if matches!(tok.token, Token::Punctuator(Punctuator::LParen)) {
            self.advance();
            let expr = self.expression();

            let TokenCtx { token, line } = self.peek();
            if !matches!(token, Token::Punctuator(Punctuator::RParen)) {
                self.errors.push(Error::new(
                    line,
                    "".into(),
                    format!("Expected closing ')' after expression, found {:?}", token),
                    ErrorKind::SyntaxError,
                ));
                self.synchronize();
            } else {
                self.advance();
            }
            Box::new(Expr::Grouping { expression: expr })
        } else {
            let TokenCtx { token, line } = self.peek();

            self.errors.push(Error::new(
                line,
                "".into(),
                format!("Expected expression, found {:?}", token),
                ErrorKind::SyntaxError,
            ));
            self.synchronize();

            Box::new(Expr::Literal {
                value: Literal::NIL,
                line: tok.line,
            })
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        let mut tok = self.peek().token;

        loop {
            if multi_matches!(
                tok,
                Token::Keyword(Keyword::CLASS)
                    | Token::Keyword(Keyword::FUN)
                    | Token::Keyword(Keyword::VAR)
                    | Token::Keyword(Keyword::FOR)
                    | Token::Keyword(Keyword::IF)
                    | Token::Keyword(Keyword::WHILE)
                    | Token::Keyword(Keyword::PRINT)
                    | Token::Keyword(Keyword::RETURN)
                    | Token::EOF
            ) {
                return;
            }

            if matches!(tok, Token::Punctuator(Punctuator::SEMICOLON)) {
                self.advance();
                return;
            }

            self.advance();
            tok = self.peek().token;
        }
    }
}
