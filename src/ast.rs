use crate::primitive::Type;

#[derive(Debug, PartialEq)]
pub enum AST {
    Number(i32),
    BinOP(Box<AST>, char, Box<AST>),
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

#[derive(Debug, PartialEq)]
pub struct Member {
    pub typ: Type,
    pub field: String,
}
