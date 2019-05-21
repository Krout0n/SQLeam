use crate::ast::{Member, AST};

type Identifier = String;
type Values = Vec<Value>;

#[derive(Debug)]
pub struct Table {
    name: Identifier,
    members: Vec<Member>,
    column: Vec<Values>,
}

#[derive(Debug, PartialEq)]
enum Value {
    Int(i32),
}

impl Value {
    fn from_ast(tree: AST) -> Self {
        match tree {
            AST::Number(i) => Value::Int(i),
            _ => unimplemented!(),
        }
    }
}

impl Table {
    pub fn new(name: Identifier, members: Vec<Member>) -> Self {
        Self {
            name,
            members,
            column: vec![],
        }
    }

    fn eval_args(args: Vec<AST>) -> Values {
        let mut column = vec![];
        for tree in args.into_iter() {
            column.push(Value::from_ast(tree));
        }
        column
    }

    pub fn execute(&mut self, name: Identifier, args: Vec<AST>) {
        match &*name {
            "insert" => {
                let args = Self::eval_args(args);
                self.column.push(args);
            }
            _ => unimplemented!(),
        }
    }
}

#[test]
fn new() {
    let t = Table::new("NewUser".to_string(), vec![]);
    assert_eq!(t.column.len(), 0);
}

#[test]
fn eval_args() {
    let args = vec![AST::Number(1), AST::Number(2)];
    assert_eq!(Table::eval_args(args), vec![Value::Int(1), Value::Int(2)]);
}
