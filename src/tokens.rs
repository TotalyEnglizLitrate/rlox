use std::{iter::Peekable, str::Chars};

use crate::error::{Error, ErrorKind};

#[derive(Debug)]
pub enum Token {
    Punctuator(Punctuator),
    Operator(Operator),
    Literal(Literal),
    Keyword(Keyword),
    EOF,
}

#[derive(Debug)]
pub enum Punctuator {
    LParen,
    RParen,
    LBrace,
    RBrace,
    COMMA,
    SEMICOLON,
    SPACE,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Literal {
    IDENT(String),
    STRING(String),
    NUMBER(f64),
}

#[derive(Debug)]
pub enum Keyword {
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
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

impl Token {
    pub fn from_str(src: &str) -> Result<Vec<Self>, Vec<Error>> {
        let lines = src.split("\n").collect::<Vec<_>>();
        let mut src_iter = src.chars().peekable();
        let mut tokens = vec![];
        let mut ctx = None;
        let mut line: usize = 1;
        let mut errors = vec![];

        'main: while let Some(c) = src_iter.next() {
            match c {
                ' ' | '\r' | '\t' => {
                    if let Some(Token::Punctuator(Punctuator::SPACE)) = tokens.last() {
                    } else {
                        tokens.push(Token::Punctuator(Punctuator::SPACE));
                    }
                    continue;
                }
                '\n' => {
                    line += 1;
                    continue;
                }
                _ => (),
            }

            match Token::parse_char(c, ctx) {
                ParseCharResult::Tokens(toks) => {
                    ctx = None;
                    tokens.extend(toks)
                }
                ParseCharResult::ADDCTX(toks) => {
                    ctx = Some(c);
                    tokens.extend(toks);
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
                    tokens.extend(toks);
                    match Token::parse_string(&mut src_iter) {
                        Ok(token) => tokens.push(token),
                        Err(mut error) => {
                            line += error.line;
                            error.content = lines[line - 1 - error.line].into();
                            error.line = line - error.line;
                            errors.push(error);
                        }
                    }
                }

                ParseCharResult::IdentOrKeyword(toks) => {
                    tokens.extend(toks);
                    match Token::parse_ident_or_keyword(&mut src_iter, c) {
                        Ok(token) => tokens.push(token),
                        Err(mut error) => {
                            error.line = line;
                            error.content = lines[line - 1].into();
                            errors.push(error);
                        }
                    }
                }

                ParseCharResult::NUM(toks) => {
                    tokens.extend(toks);
                    match Token::parse_number(&mut src_iter, c) {
                        Ok(token) => tokens.push(token),
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
            "false" => Ok(Token::Keyword(Keyword::FALSE)),
            "fun" => Ok(Token::Keyword(Keyword::FUN)),
            "for" => Ok(Token::Keyword(Keyword::FOR)),
            "if" => Ok(Token::Keyword(Keyword::IF)),
            "nil" => Ok(Token::Keyword(Keyword::NIL)),
            "or" => Ok(Token::Keyword(Keyword::OR)),
            "print" => Ok(Token::Keyword(Keyword::PRINT)),
            "return" => Ok(Token::Keyword(Keyword::RETURN)),
            "super" => Ok(Token::Keyword(Keyword::SUPER)),
            "this" => Ok(Token::Keyword(Keyword::THIS)),
            "true" => Ok(Token::Keyword(Keyword::TRUE)),
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
