use std::env;

mod lexer;
mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("请输入要查询的称呼,例如：`爸爸的爸爸是什么`");
        return;
    }
    let content = &args[1];

    let result = parse::parse(content);

    match result {
        Ok(app) => {
            println!("查询成功");
            println!("{}", app);
        }
        Err(e) => {
            eprintln!("查询失败, info: {}", e);
        }
    }
}
