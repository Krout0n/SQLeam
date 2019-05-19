#[derive(Debug, PartialEq)]
pub enum AST {
    Number(i32),
    BinOP(Box<AST>, char, Box<AST>),
}
