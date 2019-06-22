use bincode;
use statikk_db::database::Database;
use statikk_db::parser::Parser;
use statikk_db::tokenizer::Tokenizer;
use std::fs;
use std::io::{self, Read, Write};

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
            "save\n" => {
                db.save();
                break;
            }
            "read\n" => {
                let mut f = fs::File::open("db.dump").unwrap();
                let mut buf = vec![];
                f.read_to_end(&mut buf).unwrap();
                let decoded: Database = bincode::deserialize(&buf[..]).unwrap();
                db = dbg!(decoded);
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
