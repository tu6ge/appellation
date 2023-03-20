use super::lexer::{lexer, TokenKind};
use core::slice::Iter;

#[derive(Debug)]
pub(crate) struct Relation<'a> {
    first: &'a str,
    second: &'a str,
    result: &'a str,
}

impl<'a> Relation<'a> {
    fn parse(first: &'a str, list: &mut Iter<TokenKind<'a>>) -> Result<Relation<'a>, &'static str> {
        eat_link(list)?;
        let second = eat_name(list).ok_or("找不到第二个称呼")?;

        eat_is(list)?;
        let result = eat_name(list).ok_or("找不到结论称呼")?;

        Ok(Relation {
            first,
            second,
            result,
        })
    }
}

fn advance_token<'a>(list: &mut Iter<TokenKind<'a>>) -> Result<Vec<Relation<'a>>, &'static str> {
    let mut res_list = Vec::new();
    loop {
        match list.next() {
            Some(TokenKind::Comment) | Some(TokenKind::Whitespace) => continue,
            Some(TokenKind::Name(first)) => {
                res_list.push(Relation::parse(first, list)?);

                eat_delimiter(list);
            }
            Some(TokenKind::Delimiter) | Some(TokenKind::Link) | Some(TokenKind::Is) => {
                return Err("语法错误");
            }
            Some(TokenKind::Eof) => break,
            Some(TokenKind::Unknown) => return Err("未知字符"),
            None => return Err("语法错误"),
        }
    }
    Ok(res_list)
}

use crate::parse::Relation as RelationMap;

pub(crate) fn parse_str<'a>(source: &'a str) -> Result<RelationMap<'a>, &'static str> {
    let tokens = lexer(source);
    //println!("{:?}", tokens);
    let mut tokens = tokens.iter();

    let list = advance_token(&mut tokens)?;
    let iter = list
        .into_iter()
        .map(|re| ((re.first, re.second), re.result));

    Ok(RelationMap::from_iter(iter))
}

///////////////////////////////////////////////// 辅助函数 ////////////////////////////////////////

// fn first<'a>(list: &mut Iter<TokenKind<'a>>) -> Option<TokenKind<'a>> {
//    .map(|e| e.clone())
// }

fn eat_if_whitespace<'a>(list: &mut Iter<TokenKind<'a>>) {
    //let mut iter = list.peekable();
    if let Some(TokenKind::Whitespace) = list.clone().next() {
        list.next();
    }
}
fn eat_name<'a>(list: &mut Iter<TokenKind<'a>>) -> Option<&'a str> {
    eat_if_whitespace(list);
    if let Some(TokenKind::Name(str)) = list.next() {
        return Some(str);
    }
    None
}

fn eat_link<'a>(list: &mut Iter<TokenKind<'a>>) -> Result<(), &'static str> {
    eat_if_whitespace(list);
    if let Some(TokenKind::Link) = list.next() {
        return Ok(());
    }
    Err("找不到连接词")
}

fn eat_is<'a>(list: &mut Iter<TokenKind<'a>>) -> Result<(), &'static str> {
    eat_if_whitespace(list);
    if let Some(TokenKind::Is) = list.next() {
        return Ok(());
    }
    Err("找不到结论词")
}

fn eat_delimiter<'a>(list: &mut Iter<TokenKind<'a>>) {
    eat_if_whitespace(list);
    if let Some(TokenKind::Delimiter) = list.clone().next() {
        // 遇到分隔符，则吃掉
        list.next();
    }
}

#[cfg(test)]
mod tests {

    use super::parse_str;

    #[test]
    fn test_parse() {
        let str = r#"爸爸 > 妈妈 = 爷爷 
        爸爸 > 爸爸 = 爷爷"#;
        let res = parse_str(str).unwrap();
        //println!("{:?}", res);
        assert!(res.len() == 2);
    }
}
