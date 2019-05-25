use statikk_db::database::Database;
use statikk_db::parser::Parser;
use statikk_db::tokenizer::Tokenizer;
use std::io::{self, Write};

fn read() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("failed to read!");
    s
}

fn main() {
    let mut db = Database::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let s = read();
        match &*s {
            "exit\n" => break,
            "show db;\n" => {
                dbg!(&db);
                continue;
            }
            _ => (),
        };
        let tokens = Tokenizer::new(s.trim()).lex_all();
        let tree = Parser::new(tokens).parse();
        if let Err(msg) = db.execute(tree) {
            println!("{}", msg);
        } else {
            dbg!(&db);
        }
    }
}
