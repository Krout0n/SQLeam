use crate::token::{KeywordKind, Token};

#[derive(Debug, PartialEq)]
pub enum Type {
    Int,
    Chars,
}

impl Type {
    pub fn from_token(t: Token) -> Self {
        match t {
            Token::Keyword(KeywordKind::Int) => Type::Int,
            Token::Keyword(KeywordKind::Chars) => Type::Chars,
            _ => unimplemented!(),
        }
    }
}
