use std::{env, io};

use parse::{Relation, INNER_RELATION};

mod lexer;
mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut content = String::new();

    let has_arg = args.len() > 1;
    if has_arg == false {
        println!("请输入要查询的称呼,例如：`爸爸的爸爸是什么`");
    }

    let mut names_map: Relation = Relation::new();

    loop {
        if has_arg == false {
            io::stdin().read_line(&mut content).unwrap();
        } else {
            content = args[1].to_owned();
        }

        let result = parse::parse(&content, &mut names_map);

        match result {
            Ok(app) => {
                println!("查询结果：{}", app);
            }
            Err(e) => {
                eprintln!("查询失败, info: {}", e);
            }
        }
        if has_arg {
            break;
        }
    }
}
