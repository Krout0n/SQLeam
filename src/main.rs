use self::AST::*;
use statikk_db::ast::AST;
use statikk_db::parser::Parser;
use statikk_db::table::Table;
use statikk_db::tokenizer::Tokenizer;

fn read() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("failed to read!");
    s
}

fn main() {
    let mut t = Table::new();
    loop {
        let s = read();
        let tokens = Tokenizer::new(s.trim()).lex_all();
        let tree = Parser::new(tokens).parse();
        match tree {
            MethodCall { table, name, args } => t.command(&name),
            _ => {
                dbg!(tree);
                ()
            }
        };
    }
}
