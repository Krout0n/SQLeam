#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Ident(String),
    Symbol(char),
    StrLiteral(String),
    Keyword(KeywordKind),
}

#[derive(Debug, PartialEq)]
pub enum KeywordKind {
    Table,
    Int,
    Chars,
}

use KeywordKind::*;
use Token::*;

impl Token {
    pub fn lookup(literal: String) -> Self {
        match &*literal {
            "Table" => Keyword(Table),
            "int" => Keyword(Int),
            "string" => Keyword(Chars),
            _ => Ident(literal),
        }
    }
}
