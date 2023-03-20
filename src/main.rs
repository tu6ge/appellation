use std::{env, fs};

use parse::{print_error_message, ParserError};
use record::parse_str;

mod lexer;
mod parse;
mod record;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("请输入要查询的称呼,例如：`爸爸的爸爸是什么`");
        return;
    }

    let content = &args[1];

    let file_content = fs::read_to_string("./data.apl").expect("read data.apl failed");

    let map_data = parse_str(&file_content).unwrap();

    let result = parse::parse(content, map_data);

    match result {
        Ok(app) => {
            println!("查询成功");
            println!("{}", app);
        }
        Err(ParserError::NoResult) => {
            println!("查询失败，找不到匹配的结果");
        }
        Err(ParserError::Lexer { span, message }) => {
            println!("语法错误:");
            print_error_message(content, span, &message);
        }
    }
}
