use self::AST::*;
use statikk_db::ast::AST;
use statikk_db::parser::Parser;
use statikk_db::table::Table;
use statikk_db::tokenizer::Tokenizer;
use std::io::{self, Write};

fn read() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("failed to read!");
    s
}

fn main() {
    let mut t = Table::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let s = read();
        if &s == "exit\n" {
            break;
        }
        let tokens = Tokenizer::new(s.trim()).lex_all();
        let tree = Parser::new(tokens).parse();
        match tree {
            MethodCall {
                table: _,
                name: _,
                args: _,
            } => t.command(tree),
            _ => {
                dbg!(tree);
                ()
            }
        };
    }
}
