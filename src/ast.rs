use crate::primitive::Type;
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum AST {
    Number(i32),
    StrLiteral(String),
    BinOP(Box<AST>, OP, Box<AST>),
    MethodCall {
        table: String,
        name: String,
        args: Vec<AST>,
    },
    TableDef {
        name: String,
        members: Vec<Member>,
    },
}

impl AST {
    pub fn binop(left: Self, op: OP, right: Self) -> Self {
        AST::BinOP(Box::new(left), op, Box::new(right))
    }
}

#[derive(Debug, PartialEq)]
pub enum OP {
    EqEq,
    Add,
    Minus,
    Mul,
    Div,
}

impl OP {
    pub fn from_token(token: Token) -> Self {
        match token {
            Token::EqEq => OP::EqEq,
            Token::Add => OP::Add,
            Token::Minus => OP::Minus,
            Token::Mul => OP::Mul,
            Token::Slash => OP::Div,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Member {
    pub typ: Type,
    pub field: String,
}
