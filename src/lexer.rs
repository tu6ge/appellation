use std::fmt::Display;

pub(crate) const EOF_CHAR: char = '\0';

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind<'a> {
    /// `的` in `爸爸的爸爸`
    Link,
    /// `是` in `爸爸的爸爸是什么`
    Is,
    /// 什么
    What,
    /// 标识符， `爸爸`, `妈妈`等
    Ident(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token<'a> {
    kind: TokenKind<'a>,
    span: Span,
}

impl<'a> Token<'a> {
    fn new(kind: TokenKind<'a>, start: usize) -> Token<'a> {
        Token {
            kind,
            span: Span { start, length: 3 },
        }
    }

    pub(crate) fn kind(&self) -> &TokenKind<'a> {
        &self.kind
    }

    #[cfg(test)]
    fn is_kind<'b>(&self, kind: TokenKind<'b>) -> bool {
        self.kind == kind
    }

    pub(crate) fn end_span(&self) -> usize {
        self.span.end()
    }
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

pub(crate) fn check_keyword(str: char) -> bool {
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
            | '女'
            | '儿'
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    start: usize,
    length: usize,
}

impl Span {
    pub fn len(&self) -> usize {
        1.max((self.length) / 3 * 2)
    }

    pub fn end(&self) -> usize {
        self.start + self.length
    }

    pub fn print_space(&self) {
        let str = " ".repeat(self.start / 3 * 2);
        print!("{str}");
    }

    pub fn print_ref(&self) {
        let str = "^".repeat(self.len());
        print!("\x1b[0;31m");
        print!("{str}");
        print!("\x1b[0m");
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    pub(crate) span: Span,
    pub(crate) message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error {
    pub(crate) fn new(message: &str, start: usize, length: usize) -> Error {
        Self {
            span: Span { start, length },
            message: message.into(),
        }
    }

    pub(crate) fn from_token(message: &str, token: &Token) -> Error {
        Self {
            span: token.span.clone(),
            message: message.into(),
        }
    }
}

pub fn lexer<'a>(source: &'a str) -> Result<Vec<Token<'a>>, Error> {
    let mut indices = source.char_indices();

    let mut tokens = Vec::new();

    loop {
        let char_indice = indices.next();

        let (start_usize, char) = match char_indice {
            None => break,
            Some(res) => res,
        };

        let token = match char {
            _ if char.len_utf8() != 3_usize => {
                return Err(Error::new("只允许汉字", start_usize, char.len_utf8()))
            }
            '的' => Token::new(TokenKind::Link, start_usize),
            '是' => Token::new(TokenKind::Is, start_usize),
            '什' => {
                let mut iter = indices.clone().peekable();
                match iter.peek() {
                    Some((_, con)) => {
                        if con == &'么' {
                            indices.next();
                            Token::new(TokenKind::What, start_usize)
                        } else {
                            return Err(Error::new("期望在 `什` 后面是的 `么`", start_usize, 3));
                        }
                    }
                    None => return Err(Error::new("不能以 `什` 作为结尾", start_usize, 3)),
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
                                t = Token::new(
                                    TokenKind::Ident(&source[start_usize..last_usize]),
                                    start_usize,
                                );
                                break;
                            } else {
                                indices.next();
                            }
                        }
                        None => {
                            t = Token::new(
                                TokenKind::Ident(&source[start_usize..current_usize + 3]),
                                start_usize,
                            );
                            break;
                        }
                    }
                }
                t
            }
            _ => return Err(Error::new("未定义字符", start_usize, char.len_utf8())),
        };

        tokens.push(token);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let res = lexer("的").unwrap();
        assert_eq!(res[0].kind(), &TokenKind::Link);

        let res = lexer("是").unwrap();
        assert_eq!(res[0].kind(), &TokenKind::Is);

        let res = lexer("什么").unwrap();
        assert_eq!(res[0].kind(), &TokenKind::What);

        let res = lexer("爸爸").unwrap();
        assert_eq!(res[0].kind(), &TokenKind::Ident("爸爸"));

        let res = lexer("爸叔").unwrap();
        assert_eq!(res[0].kind(), &TokenKind::Ident("爸叔"));

        let res = lexer("爸爸的爷爷").unwrap();
        assert!(res[0].is_kind(TokenKind::Ident("爸爸")));
        assert!(res[1].is_kind(TokenKind::Link));
        assert!(res[2].is_kind(TokenKind::Ident("爷爷")));
    }
}
