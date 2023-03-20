use std::str::CharIndices;

use crate::lexer::{check_keyword, Span, EOF_CHAR};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind<'a> {
    /// Any whitespace character sequence.
    Whitespace,
    /// `>`
    Link,
    /// `=`
    Is,
    /// `爸爸`
    Name(&'a str),
    /// `\r` `\r\n`
    Delimiter,
    /// 注释
    Comment,
    /// 未知字符
    Unknown,
    /// 结束
    Eof,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token<'a> {
    kind: TokenKind<'a>,
    span: Span,
}

/// 判断字符是否是空白
fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{0020}' // space

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

/// 判断字符是否是换行
fn is_delimiter(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{000A}' // \n
        | '\u{000D}' // \r

        // NEXT LINE from latin1
        | '\u{0085}'

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
    )
}

fn first(indices: &CharIndices) -> char {
    // `.next()` optimizes better than `.nth(0)`
    indices
        .clone()
        .next()
        .map(|(_, str)| str)
        .unwrap_or(EOF_CHAR)
}

fn is_eof(indices: &CharIndices) -> bool {
    indices.as_str().is_empty()
}

/// Eats symbols while predicate returns true or until the end of file is reached.
fn eat_while(indices: &mut CharIndices, mut predicate: impl FnMut(char) -> bool) {
    // It was tried making optimized version of this for eg. line comments, but
    // LLVM can inline all of this and compile it down to fast iteration over bytes.
    while predicate(first(indices)) && !is_eof(indices) {
        indices.next();
    }
}

/// 返回最后一个字的偏移量
fn eat_while2(indices: &mut CharIndices, mut predicate: impl FnMut(char) -> bool) -> usize {
    let mut offset = 0_usize;
    // It was tried making optimized version of this for eg. line comments, but
    // LLVM can inline all of this and compile it down to fast iteration over bytes.
    while predicate(first(indices)) && !is_eof(indices) {
        if let Some((o, _)) = indices.next() {
            offset = o;
        }
    }
    offset
}

pub fn lexer<'a>(source: &'a str) -> Vec<TokenKind<'a>> {
    let mut indices = source.char_indices();

    let mut tokens = Vec::new();

    loop {
        let char_indice = indices.next();

        let (start_usize, char) = match char_indice {
            None => {
                tokens.push(TokenKind::Eof);
                break;
            }
            Some(res) => res,
        };

        let token = match char {
            '#' => {
                eat_while(&mut indices, |c| c != '\n');
                indices.next(); // 吃掉 \n
                TokenKind::Comment
            }
            _ if is_whitespace(char) => {
                eat_while(&mut indices, is_whitespace);
                TokenKind::Whitespace
            }
            _ if is_delimiter(char) => {
                eat_while(&mut indices, is_delimiter);
                TokenKind::Delimiter
            }
            '>' => TokenKind::Link,
            '=' => TokenKind::Is,
            _ if check_keyword(char) => {
                let last_offset = eat_while2(&mut indices, check_keyword);
                TokenKind::Name(&source[start_usize..last_offset + 3])
            }
            _ => TokenKind::Unknown,
        };

        tokens.push(token);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base() {
        let tokens = lexer("#abc\n \t   \n\r\n\r\n\n爷爷爷 > 爸爸");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Comment,
                TokenKind::Whitespace,
                TokenKind::Delimiter,
                TokenKind::Name("爷爷爷"),
                TokenKind::Whitespace,
                TokenKind::Link,
                TokenKind::Whitespace,
                TokenKind::Name("爸爸"),
                TokenKind::Eof,
            ]
        );

        let tokens = lexer("#abc");
        assert_eq!(tokens, vec![TokenKind::Comment, TokenKind::Eof]);

        let tokens = lexer("郑");
        assert_eq!(tokens, vec![TokenKind::Unknown, TokenKind::Eof]);

        let tokens = lexer("");
        assert_eq!(tokens, vec![TokenKind::Eof]);
    }
}
