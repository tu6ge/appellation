use std::{collections::HashMap, fmt::Display};

use crate::lexer::{lexer, Token};
use lazy_static::lazy_static;

pub type Relation = HashMap<(&'static str, &'static str), &'static str>;

lazy_static! {
    pub static ref INNER_RELATION: Relation = {
        let mut map = Relation::new();
        map.insert(("爸爸", "爸爸"), "爷爷");
        map.insert(("爸爸", "妈妈"), "奶奶");
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

pub fn parse<'a, 'b>(
    source: &'a str,
    map: &'b mut Relation,
) -> Result<Appellation<'a>, &'static str> {
    let tokens = lexer(source)?;
    let mut tokens = tokens.iter();

    let first = tokens.next().ok_or("找不到第一个称呼")?;
    let link = tokens.next().ok_or("找不到连接词")?;
    let second = tokens.next().ok_or("找不到第二个称呼")?;
    let is = tokens.next().ok_or("找不到 `是`")?;
    let what = tokens.next().ok_or("找不到 `什么`")?;

    let first = if let Token::Ident(name) = first {
        name
    } else {
        return Err("语法错误，期望一个称呼");
    };

    let second = if let Token::Ident(name) = second {
        name
    } else {
        return Err("语法错误，期望一个称呼");
    };

    match link {
        Token::Link => (),
        _ => return Err("语法错误，期望`的`"),
    }

    match is {
        Token::Is => (),
        _ => return Err("语法错误，期望 `是`"),
    }

    match what {
        Token::What => (),
        Token::Ident(result) => {
            map.insert((first.to_owned(), second.to_owned()), result.to_owned());
        }
        _ => return Err("语法错误，期望 `什么`"),
    };

    //

    match map.get(&(first, second)) {
        Some(result) => Ok(Appellation {
            first,
            second,
            result,
        }),
        None => Err("找不到结果"),
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::Relation;

    use super::parse;

    #[test]
    fn yeye() {
        let mut map = Relation::new();
        let res = parse("爸爸的爸爸是什么", &mut map).unwrap();

        assert_eq!(res.result(), "爷爷");

        //println!("{}", res);
    }
}
