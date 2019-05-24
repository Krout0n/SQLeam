use crate::ast::{Member, AST};
use crate::primitive::Type;

type Identifier = String;
type Values = Vec<Value>;

// TODO: Rename better and Retype.
type R = Result<(), &'static str>;

#[derive(Debug)]
pub struct Table {
    name: Identifier,
    members: Vec<Member>,
    column: Vec<Values>,
}

#[derive(Debug, PartialEq)]
enum Value {
    Int(i32),
    StrLiteral(String),
}

impl Value {
    fn from_ast(tree: AST) -> Self {
        match tree {
            AST::Number(i) => Value::Int(i),
            AST::StrLiteral(s) => Value::StrLiteral(s),
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

    fn type_check(&self, args: &Values) -> R {
        if args.len() != self.members.len() {
            return Err("Wrong number of arguments.");
        }
        for (index, arg) in args.iter().enumerate() {
            let typ = &self.members.get(index).unwrap().typ;
            match (typ, arg) {
                (Type::Int, Value::Int(_)) | (Type::Chars, Value::StrLiteral(_)) => (),
                _ => return Err("Unmatched type of arg."),
            };
        }
        Ok(())
    }

    pub fn execute(&mut self, name: Identifier, args: Vec<AST>) -> R {
        match &*name {
            "insert" => {
                let args = Self::eval_args(args);
                self.type_check(&args)?;
                self.column.push(args);
                Ok(())
            }
            "delete" => {
                if let Some(arg) = Self::eval_args(args).get(0) {
                    if let Value::Int(index) = arg {
                        self.column.remove(*index as usize + 1);
                    }
                }
                Ok(())
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

    let args = vec![AST::Number(1), AST::StrLiteral("kuru".to_string())];
    assert_eq!(
        Table::eval_args(args),
        vec![Value::Int(1), Value::StrLiteral("kuru".to_string())]
    );
}

#[test]
fn type_check() {
    let table = Table::new(
        "NewUser".to_string(),
        vec![Member {
            field: "id".to_string(),
            typ: Type::Int,
        }],
    );

    assert_eq!(table.type_check(&vec![]), Err("Wrong number of arguments."));

    assert_eq!(table.type_check(&vec![Value::Int(10)]), Ok(()));

    assert_eq!(
        table.type_check(&vec![Value::StrLiteral("hoge".to_string())]),
        Err("Unmatched type of arg.")
    );
}
