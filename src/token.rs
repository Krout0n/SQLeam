#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Ident(String),
    Symbol(char),
    Keyword(KeywordKind),
}

#[derive(Debug, PartialEq)]
pub enum KeywordKind {
    Table,
    Int,
}

use KeywordKind::*;
use Token::*;

impl Token {
    pub fn lookup(literal: String) -> Self {
        match &*literal {
            "Table" => Keyword(Table),
            "int" => Keyword(Int),
            _ => Ident(literal),
        }
    }
}
