use statikk_db::parser::Parser;
use statikk_db::tokenizer::Tokenizer;

fn read() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("failed to read!");
    s
}

fn main() {
    loop {
        let s = read();
        let tokens = Tokenizer::new(s.trim()).lex_all();
        dbg!(Parser::new(tokens).parse());
    }
}
