use std::{
    iter::Peekable,
    ops::{Add, Div, Mul, Neg, Not, Sub},
    str::Chars,
};

use crate::error::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct TokenCtx {
    pub(crate) token: Token,
    pub(crate) line: usize,
}

#[derive(Debug, Clone)]
pub enum Token {
    Punctuator(Punctuator),
    Operator(Operator),
    Literal(Literal),
    Keyword(Keyword),
    EndGrouping,
    ParseError,
    EOF,
}

#[derive(Debug, Clone)]
pub enum Punctuator {
    LParen,
    RParen,
    LBrace,
    RBrace,
    COMMA,
    SEMICOLON,
}

#[derive(Debug, Clone)]
pub enum Operator {
    DOT,
    MINUS,
    PLUS,
    SLASH,
    STAR,
    BANG,
    ASSIGN,
    EQ,
    NE,
    GREATER,
    GE,
    LESSER,
    LE,
}

#[derive(Debug, Clone)]
pub enum Literal {
    TRUE,
    FALSE,
    NIL,
    IDENT(String),
    STRING(String),
    NUMBER(f64),
}

#[derive(Debug, Clone)]
pub enum Keyword {
    AND,
    CLASS,
    ELSE,
    FUN,
    FOR,
    IF,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    VAR,
    WHILE,
}

enum ParseCharResult {
    INVALID(Error),
    Tokens(Vec<Token>),
    COMMENT,
    ADDCTX(Vec<Token>),
    STRING(Vec<Token>),
    IdentOrKeyword(Vec<Token>),
    NUM(Vec<Token>),
}

impl TokenCtx {
    pub fn from_str(src: &str) -> Result<Vec<Self>, Vec<Error>> {
        let lines = src.split("\n").collect::<Vec<_>>();
        let mut src_iter = src.chars().peekable();
        let mut tokens: Vec<Self> = vec![];
        let mut ctx = None;
        let mut line: usize = 1;
        let mut errors = vec![];

        'main: while let Some(c) = src_iter.next() {
            match c {
                ' ' | '\r' | '\t' => {
                    continue;
                }
                '\n' => {
                    line += 1;
                    continue;
                }
                _ => (),
            }

            match TokenCtx::parse_char(c, ctx) {
                ParseCharResult::Tokens(toks) => {
                    ctx = None;
                    tokens.extend(toks.iter().map(|t| TokenCtx {
                        token: t.to_owned(),
                        line,
                    }));
                }
                ParseCharResult::ADDCTX(toks) => {
                    ctx = Some(c);
                    tokens.extend(toks.iter().map(|t| TokenCtx {
                        token: t.to_owned(),
                        line,
                    }));
                }

                ParseCharResult::INVALID(mut error) => {
                    error.line = line;
                    error.content = lines[line - 1].into();
                    errors.push(error);
                }

                ParseCharResult::COMMENT => {
                    ctx = None;
                    while let Some(c) = src_iter.next() {
                        if c == '\n' {
                            line += 1;
                            continue 'main;
                        }
                    }
                    break 'main;
                }

                ParseCharResult::STRING(toks) => {
                    ctx = None;
                    tokens.extend(toks.iter().map(|t| TokenCtx {
                        token: t.to_owned(),
                        line,
                    }));
                    match TokenCtx::parse_string(&mut src_iter) {
                        Ok(token) => tokens.push(TokenCtx { token, line }),
                        Err(mut error) => {
                            line += error.line;
                            error.content = lines[line - 1 - error.line].into();
                            error.line = line - error.line;
                            errors.push(error);
                        }
                    }
                }

                ParseCharResult::IdentOrKeyword(toks) => {
                    ctx = None;
                    tokens.extend(toks.iter().map(|t| TokenCtx {
                        token: t.to_owned(),
                        line,
                    }));
                    match TokenCtx::parse_ident_or_keyword(&mut src_iter, c) {
                        Ok(token) => tokens.push(TokenCtx { token, line }),
                        Err(mut error) => {
                            error.line = line;
                            error.content = lines[line - 1].into();
                            errors.push(error);
                        }
                    }
                }

                ParseCharResult::NUM(toks) => {
                    ctx = None;
                    tokens.extend(toks.iter().map(|t| TokenCtx {
                        token: t.to_owned(),
                        line,
                    }));
                    match TokenCtx::parse_number(&mut src_iter, c) {
                        Ok(token) => tokens.push(TokenCtx { token, line }),
                        Err(mut error) => {
                            error.line = line;
                            error.content = lines[line - 1].into();
                            errors.push(error);
                        }
                    }
                }
            }
        }

        if errors.len() != 0 {
            Err(errors)
        } else {
            if !matches!(
                tokens.last().map(|TokenCtx { token, line: _ }| token),
                Some(Token::EOF)
            ) {
                tokens.push(TokenCtx {
                    token: Token::EOF,
                    line,
                });
            }
            Ok(tokens)
        }
    }

    fn parse_number(src_iter: &mut Peekable<Chars>, first_char: char) -> Result<Token, Error> {
        let mut number = String::new();
        number.push(first_char);
        while let Some(c) = src_iter.peek() {
            if c.is_ascii_digit() || c == &'.' {
                number.push(src_iter.next().unwrap());
            } else {
                break;
            }
        }

        match number.parse::<f64>() {
            Ok(num) => Ok(Token::Literal(Literal::NUMBER(num))),
            Err(_) => Err(Error::new(
                0,
                "".into(),
                "Invalid number literal".into(),
                ErrorKind::SyntaxError,
            )),
        }
    }

    fn parse_ident_or_keyword(
        src_iter: &mut Peekable<Chars>,
        first_char: char,
    ) -> Result<Token, Error> {
        let mut ident = String::new();
        ident.push(first_char);
        while let Some(c) = src_iter.peek() {
            if c.is_ascii_alphanumeric() || c == &'_' {
                ident.push(src_iter.next().unwrap());
            } else {
                break;
            }
        }

        match ident.as_str() {
            "and" => Ok(Token::Keyword(Keyword::AND)),
            "class" => Ok(Token::Keyword(Keyword::CLASS)),
            "else" => Ok(Token::Keyword(Keyword::ELSE)),
            "false" => Ok(Token::Literal(Literal::FALSE)),
            "fun" => Ok(Token::Keyword(Keyword::FUN)),
            "for" => Ok(Token::Keyword(Keyword::FOR)),
            "if" => Ok(Token::Keyword(Keyword::IF)),
            "nil" => Ok(Token::Literal(Literal::NIL)),
            "or" => Ok(Token::Keyword(Keyword::OR)),
            "print" => Ok(Token::Keyword(Keyword::PRINT)),
            "return" => Ok(Token::Keyword(Keyword::RETURN)),
            "super" => Ok(Token::Keyword(Keyword::SUPER)),
            "this" => Ok(Token::Keyword(Keyword::THIS)),
            "true" => Ok(Token::Literal(Literal::TRUE)),
            "var" => Ok(Token::Keyword(Keyword::VAR)),
            "while" => Ok(Token::Keyword(Keyword::WHILE)),
            _ => Ok(Token::Literal(Literal::IDENT(ident))),
        }
    }

    fn parse_string(src_iter: &mut Peekable<Chars>) -> Result<Token, Error> {
        let mut s = String::new();
        while let Some(c) = src_iter.next() {
            if c == '"' {
                return Ok(Token::Literal(Literal::STRING(s)));
            }

            if c == '\n' {
                return Err(Error::new(
                    1,
                    "".into(),
                    "Unexpected newline in string literal".into(),
                    crate::error::ErrorKind::SyntaxError,
                ));
            }
            s.push(c);
        }

        return Err(Error::new(
            0,
            "".into(),
            "Unexpected EOF in string literal".into(),
            crate::error::ErrorKind::SyntaxError,
        ));
    }

    fn parse_char(c: char, ctx: Option<char>) -> ParseCharResult {
        let mut toks = vec![];
        if let Some(ctx) = ctx {
            match ctx {
                '/' => {
                    if c != '/' {
                        toks.push(Token::Operator(Operator::SLASH));
                    } else {
                        return ParseCharResult::COMMENT;
                    }
                }
                '!' => {
                    if c != '=' {
                        toks.push(Token::Operator(Operator::BANG));
                    } else {
                        toks.push(Token::Operator(Operator::NE));
                        return ParseCharResult::Tokens(toks);
                    }
                }

                '=' => {
                    if c != '=' {
                        toks.push(Token::Operator(Operator::ASSIGN));
                    } else {
                        toks.push(Token::Operator(Operator::EQ));
                        return ParseCharResult::Tokens(toks);
                    }
                }

                '>' => {
                    if c == '=' {
                        toks.push(Token::Operator(Operator::GE));
                        return ParseCharResult::Tokens(toks);
                    } else {
                        toks.push(Token::Operator(Operator::GREATER));
                    }
                }

                '<' => {
                    if c == '=' {
                        toks.push(Token::Operator(Operator::LE));
                        return ParseCharResult::Tokens(toks);
                    } else {
                        toks.push(Token::Operator(Operator::LESSER));
                    }
                }
                _ => unreachable!(),
            }
        }

        match c {
            '(' => toks.push(Token::Punctuator(Punctuator::LParen)),
            ')' => toks.push(Token::Punctuator(Punctuator::RParen)),
            '{' => toks.push(Token::Punctuator(Punctuator::LBrace)),
            '}' => toks.push(Token::Punctuator(Punctuator::RBrace)),
            ',' => toks.push(Token::Punctuator(Punctuator::COMMA)),
            ';' => toks.push(Token::Punctuator(Punctuator::SEMICOLON)),
            '*' => toks.push(Token::Operator(Operator::STAR)),
            '.' => toks.push(Token::Operator(Operator::DOT)),
            '-' => toks.push(Token::Operator(Operator::MINUS)),
            '+' => toks.push(Token::Operator(Operator::PLUS)),
            '\0' => toks.push(Token::EOF),
            '!' | '=' | '>' | '<' | '/' => return ParseCharResult::ADDCTX(toks),
            '"' => return ParseCharResult::STRING(toks),
            _ => {
                if c.is_ascii_digit() {
                    return ParseCharResult::NUM(toks);
                } else if c.is_ascii_alphabetic() || c == '_' {
                    return ParseCharResult::IdentOrKeyword(toks);
                } else {
                    return ParseCharResult::INVALID(Error::new(
                        0,
                        "".into(),
                        "Invalid syntax!".into(),
                        ErrorKind::SyntaxError,
                    ));
                }
            }
        }
        ParseCharResult::Tokens(toks)
    }
}

impl Literal {
    pub(crate) fn truthy(&self) -> Self {
        match self {
            Self::STRING(s) => {
                if s.is_empty() {
                    Self::TRUE
                } else {
                    Self::FALSE
                }
            }
            Self::NUMBER(num) => {
                if num == &0f64 {
                    Self::TRUE
                } else {
                    Self::FALSE
                }
            }
            Self::NIL => Self::FALSE,
            Self::IDENT(_) => unimplemented!(),
            _ => self.clone(),
        }
    }

    pub(crate) fn get_type(&self) -> String {
        (match self {
            Self::NUMBER(_) => "number",
            Self::STRING(_) => "string",
            Self::FALSE | Self::TRUE => "Boolean",
            Self::NIL => "nil",
            Self::IDENT(_) => unimplemented!()
        }).into()
    }
}

impl Not for &Literal {
    type Output = Literal;
    fn not(self) -> Literal {
        match self.truthy() {
            Literal::TRUE => Literal::FALSE,
            Literal::FALSE => Literal::TRUE,
            _ => unimplemented!(),
        }
    }
}

impl Add for &Literal {
    type Output = Option<Literal>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::NUMBER(x), Literal::NUMBER(y)) => Some(Literal::NUMBER(x + y)),
            (Literal::STRING(x), Literal::STRING(y)) => Some(Literal::STRING(x.clone() + y)),
            (Literal::IDENT(_), Literal::IDENT(_)) => unimplemented!(),
            _ => None
        }
    }
}

impl Neg for &Literal {
    type Output = Option<Literal>;
    fn neg(self) -> Self::Output {
        match self {
            Literal::NUMBER(x) => Some(Literal::NUMBER(-x)),
            _ => None
        }
    }
}

impl Sub for &Literal {
    type Output = Option<Literal>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::NUMBER(x), Literal::NUMBER(y)) => Some(Literal::NUMBER(x - y)),
            _ => None
        }
    }
}

impl Mul for &Literal {
    type Output = Option<Literal>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::NUMBER(x), Literal::NUMBER(y)) => Some(Literal::NUMBER(x * y)),
            _ => None
        }
    }
}

impl Div for &Literal {
    type Output = Option<Literal>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::NUMBER(x), Literal::NUMBER(y)) => Some(Literal::NUMBER(x / y)),
            _ => None
        }
    }
}
