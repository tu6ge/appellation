use std::env;

use parse::{print_error_message, Parser, ParserError};

mod lexer;
mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("请输入要查询的称呼,例如：`爸爸的爸爸是什么`");
        return;
    }

    let content = &args[1];

    let parser = Parser::new(content);

    let result = parser.to_appellation();

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
