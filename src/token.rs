#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Ident(String),
    Symbol(char),
    StrLiteral(String),
    Keyword(KeywordKind),
    EqEq,
    Add,
    Minus,
    Mul,
    Slash,
}

#[derive(Debug, PartialEq)]
pub enum KeywordKind {
    Table,
    Int,
    StrLiteral,
}

use KeywordKind::*;
use Token::*;

impl Token {
    pub fn lookup(literal: String) -> Self {
        match &*literal {
            "Table" => Keyword(Table),
            "int" => Keyword(Int),
            "string" => Keyword(KeywordKind::StrLiteral),
            _ => Ident(literal),
        }
    }
}
