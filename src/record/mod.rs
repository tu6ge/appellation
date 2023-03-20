mod lexer;
mod parse;

pub(crate) use self::parse::parse_str;

// use crate::parse::Relation as RelationMap;

// fn parse_file<'a,'b>(path: &'a str) -> Result<RelationMap<'b>, &'static str> {
//     let source = fs::read_to_string(path).unwrap();

//     parse_str(&source)
// }
