#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    /// `的` in `爸爸的爸爸`
    Link,
    /// `是` in `爸爸的爸爸是什么`
    Is,
    /// 什么
    What,
    /// 标识符， `爸爸`, `妈妈`等
    Ident(&'a str),
    /// 结束符
    Eof,
}

// enum Label {
//     爸爸,
//     妈妈,
//     爷爷,
//     奶奶,
//     姥姥,
//     姥爷,
//     伯伯,
//     叔叔,
//     舅舅,
//     姑姑,
//     哥哥,
//     弟弟,
//     姐姐,
//     妹妹,
// }

fn check_keyword(str: char) -> bool {
    matches!(
        str,
        '爸' | '妈'
            | '爷'
            | '奶'
            | '姑'
            | '父'
            | '母'
            | '舅'
            | '妗'
            | '子'
            | '伯'
            | '叔'
            | '婶'
            | '哥'
            | '妹'
            | '姐'
            | '弟'
            | '姥'
            | '姨'
    )
}

pub fn lexer<'a>(source: &'a str) -> Result<Vec<Token<'a>>, &'static str> {
    let mut indices = source.char_indices();

    let mut tokens = Vec::new();

    loop {
        let char_indice = indices.next();

        let (start_usize, char) = match char_indice {
            None => break,
            Some(res) => res,
        };

        let token = match char {
            '的' => Token::Link,
            '是' => Token::Is,
            '\r' => Token::Eof,
            '\n' => Token::Eof,
            '什' => {
                let mut iter = indices.clone().peekable();
                match iter.peek() {
                    Some((_, con)) => {
                        if con == &'么' {
                            indices.next();
                            Token::What
                        } else {
                            return Err("语法错误，期望在 `什` 后面是的 `么`");
                        }
                    }
                    None => return Err("语法错误，不能以 `什` 作为结尾"),
                }
            }
            _ if check_keyword(char) => {
                let mut iter = indices.clone().peekable();
                let t;
                let mut current_usize = start_usize;
                loop {
                    match iter.next() {
                        Some((last_usize, con)) => {
                            current_usize = last_usize;
                            if !check_keyword(con) {
                                t = Token::Ident(&source[start_usize..last_usize]);
                                break;
                            } else {
                                indices.next();
                            }
                        }
                        None => {
                            t = Token::Ident(&source[start_usize..current_usize + 3]);
                            break;
                        }
                    }
                }
                t
            }
            _ => return Err("未定义的字符串"),
        };

        tokens.push(token);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::{lexer, Token};

    #[test]
    fn test_lexer() {
        let res = lexer("的");
        assert_eq!(res, Ok(vec![Token::Link]));

        let res = lexer("是");
        assert_eq!(res, Ok(vec![Token::Is]));

        let res = lexer("什么");
        assert_eq!(res, Ok(vec![Token::What]));

        let res = lexer("爸爸");
        assert_eq!(res, Ok(vec![Token::Ident("爸爸")]));

        let res = lexer("爸叔");
        assert_eq!(res, Ok(vec![Token::Ident("爸叔")]));

        let res = lexer("爸爸的爷爷");
        assert_eq!(
            res,
            Ok(vec![
                Token::Ident("爸爸"),
                Token::Link,
                Token::Ident("爷爷"),
            ])
        );
    }
}
