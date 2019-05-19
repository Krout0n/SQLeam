#[derive(Debug, PartialEq)]
pub enum AST {
    Number(i32),
    BinOP(Box<AST>, char, Box<AST>),
    MethodCall {
        table: String,
        name: String,
        args: Vec<AST>,
    },
}
