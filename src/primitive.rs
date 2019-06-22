use crate::token::{KeywordKind, Token};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Type {
    Int,
    StrLiteral,
}

impl Type {
    pub fn from_token(t: Token) -> Self {
        match t {
            Token::Keyword(KeywordKind::Int) => Type::Int,
            Token::Keyword(KeywordKind::StrLiteral) => Type::StrLiteral,
            _ => unimplemented!(),
        }
    }
}
