use std::{collections::HashMap, fmt::Display};

use crate::lexer::{lexer, Error, Span, Token, TokenKind};
use core::slice::Iter;
use lazy_static::lazy_static;

type Relation = HashMap<(&'static str, &'static str), &'static str>;

lazy_static! {
    static ref INNER_RELATION: Relation = {
        let mut map = Relation::new();
        map.insert(("爸爸", "爸爸"), "爷爷");
        map.insert(("爸爸", "妈妈"), "奶奶");
        map.insert(("爸爸", "老大"), "大哥或自己");
        map
    };
}

#[derive(Debug)]
pub struct Appellation<'a> {
    first: &'a str,
    second: &'a str,
    result: &'a str,
}

impl<'a> Display for Appellation<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}的{}是{}",
            self.first, self.second, self.result
        ))
    }
}

impl<'a> Appellation<'a> {
    #[allow(dead_code)]
    pub fn result(&self) -> &'a str {
        self.result
    }
}

#[derive(Debug)]
pub enum ParserError {
    Lexer { span: Span, message: String },
    NoResult,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lexer { message, .. } => write!(f, "{}", message),
            Self::NoResult => write!(f, "找不到结果"),
        }
    }
}

impl From<Error> for ParserError {
    fn from(Error { span, message }: Error) -> Self {
        Self::Lexer { span, message }
    }
}

pub struct Parser<'a> {
    source: &'a str,
    current_offset: usize,
    tokens: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            current_offset: 0,
            tokens: Vec::default(),
        }
    }

    pub fn parse(&mut self) -> Result<(), ParserError> {
        self.tokens = lexer(self.source).map_err(ParserError::from)?;
        let tokens = self.tokens.clone();
        let mut tokens = tokens.iter();

        let first_token =
            tokens
                .next()
                .ok_or(Error::new("找不到第一个称呼", self.current_offset, 1))?;
        let first = if let TokenKind::Ident(name) = first_token.kind() {
            self.current_offset += first_token.length();
            name
        } else {
            return Err(Error::from_token("期望一个称呼", first_token).into());
        };

        let link_token = tokens
            .next()
            .ok_or(Error::new("找不到 `的`", self.current_offset, 1))?;
        match link_token.kind() {
            TokenKind::Link => {
                self.current_offset += link_token.length();
            }
            _ => return Err(Error::from_token("期望 `的`", link_token).into()),
        }

        let second = self.parser_second_name(&mut tokens)?;

        let is_token = tokens
            .next()
            .ok_or(Error::new("找不到 `是`", self.current_offset, 1))?;
        match is_token.kind() {
            TokenKind::Is => {
                self.current_offset += is_token.length();
            }
            _ => return Err(Error::from_token("期望 `是`", is_token).into()),
        }

        let what_token =
            tokens
                .next()
                .ok_or(Error::new("找不到 `什么`", self.current_offset, 1))?;
        match what_token.kind() {
            TokenKind::What => {
                self.current_offset += what_token.length();
            }
            _ => return Err(Error::from_token("语法错误，期望 `什么`", what_token).into()),
        };

        match INNER_RELATION.get(&(first, &second)) {
            Some(result) => Ok(()),
            None => Err(ParserError::NoResult),
        }
    }

    pub fn to_appellation(&self) -> Result<Appellation, ParserError> {
        //     Appellation {
        //     first,
        //     second,
        //     result,
        // }
        todo!()
    }

    fn parser_second_name<'b>(
        &'a mut self,
        tokens: &'b mut Iter<Token>,
    ) -> Result<String, ParserError> {
        let second_token =
            tokens
                .next()
                .ok_or(Error::new("找不到第二个称呼", self.current_offset, 1))?;
        let second = if let TokenKind::Ident(name) = second_token.kind() {
            self.current_offset += second_token.length();
            name
        } else {
            return Err(Error::from_token("期望一个称呼", second_token).into());
        };

        Ok(second.to_string())
    }
}

pub fn print_error_message(code: &str, span: Span, message: &str) {
    println!("{code}");
    span.print_space();
    span.print_ref();
    println!("");
    span.print_space();
    print_error(message);
}

fn print_error(msg: &str) {
    print!("\x1b[0;31m");
    print!("{}", msg);
    print!("\x1b[0m");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yeye() {
        let parser = Parser::new("爸爸的爸爸是什么");
        let res = parser.to_appellation().unwrap();

        assert_eq!(res.result(), "爷爷");

        //println!("{}", res);
    }
}
