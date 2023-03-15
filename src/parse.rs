use std::{collections::HashMap, fmt::Display};

use crate::lexer::{lexer, Error, Span, TokenKind};
use lazy_static::lazy_static;

type Relation = HashMap<(&'static str, &'static str), &'static str>;

lazy_static! {
    static ref INNER_RELATION: Relation = {
        let mut map = Relation::new();
        map.insert(("爸爸", "爸爸"), "爷爷");
        map.insert(("爸爸", "妈妈"), "奶奶");
        map.insert(("妈妈", "妈妈"), "姥姥");
        map.insert(("妈妈", "爸爸"), "姥爷");
        map.insert(("爸爸", "哥哥"), "伯伯");
        map.insert(("爸爸", "弟弟"), "叔叔");
        map.insert(("爸爸", "妹妹"), "姑姑");
        map.insert(("爸爸", "姐姐"), "姑姑");
        map.insert(("哥哥", "儿子"), "侄子");
        map.insert(("哥哥", "女儿"), "侄女");
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

pub fn parse<'a>(source: &'a str) -> Result<Appellation<'a>, ParserError> {
    let tokens = lexer(source).map_err(ParserError::from)?;
    let mut tokens = tokens.iter();

    let first_token = tokens.next().ok_or(Error::new("找不到第一个称呼", 0, 1))?;
    let first = if let TokenKind::Ident(name) = first_token.kind() {
        name
    } else {
        return Err(Error::from_token("期望一个称呼", first_token).into());
    };

    let link_token = tokens
        .next()
        .ok_or(Error::new("找不到 `的`", first_token.end_span(), 1))?;
    match link_token.kind() {
        TokenKind::Link => (),
        _ => return Err(Error::from_token("期望 `的`", link_token).into()),
    }

    let second_token =
        tokens
            .next()
            .ok_or(Error::new("找不到第二个称呼", link_token.end_span(), 1))?;
    let second = if let TokenKind::Ident(name) = second_token.kind() {
        name
    } else {
        return Err(Error::from_token("期望一个称呼", second_token).into());
    };

    let is_token = tokens
        .next()
        .ok_or(Error::new("找不到 `是`", second_token.end_span(), 1))?;
    match is_token.kind() {
        TokenKind::Is => (),
        _ => return Err(Error::from_token("期望 `是`", is_token).into()),
    }

    let what_token = tokens
        .next()
        .ok_or(Error::new("找不到 `什么`", is_token.end_span(), 1))?;
    match what_token.kind() {
        TokenKind::What => (),
        _ => return Err(Error::from_token("语法错误，期望 `什么`", what_token).into()),
    };

    match INNER_RELATION.get(&(first, second)) {
        Some(result) => Ok(Appellation {
            first,
            second,
            result,
        }),
        None => Err(ParserError::NoResult),
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
    use super::parse;

    #[test]
    fn yeye() {
        let res = parse("爸爸的爸爸是什么").unwrap();

        assert_eq!(res.result(), "爷爷");

        //println!("{}", res);
    }
}
