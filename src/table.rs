use crate::ast::{Member, AST, OP};
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
    Bool(bool),
}

impl Value {
    fn eval_ast(tree: AST) -> Self {
        match tree {
            AST::Number(i) => Value::Int(i),
            AST::StrLiteral(s) => Value::StrLiteral(s),
            AST::BinOP(left, op, right) => {
                let left = Self::eval_ast(*left);
                let right = Self::eval_ast(*right);
                match (left, op, right) {
                    (Value::Int(lhs), OP::Add, Value::Int(rhs)) => Value::Int(lhs + rhs),
                    (Value::Int(lhs), OP::Minus, Value::Int(rhs)) => Value::Int(lhs - rhs),
                    (Value::Int(lhs), OP::Mul, Value::Int(rhs)) => Value::Int(lhs * rhs),
                    (Value::Int(lhs), OP::EqEq, Value::Int(rhs)) => Value::Bool(lhs == rhs),
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}

macro_rules! type_value {
    ($variant: ident) => {
        (Type::$variant, Value::$variant(_))
    };
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
            column.push(Value::eval_ast(tree));
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
                type_value!(Int) | type_value!(StrLiteral) => (),
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

#[test]
fn eval_ast() {
    let ast = AST::binop(AST::Number(1), OP::Add, AST::Number(2));
    assert_eq!(Value::eval_ast(ast), Value::Int(3));

    let ast = AST::binop(
        AST::binop(AST::Number(1), OP::Add, AST::Number(2)),
        OP::Add,
        AST::Number(3),
    );
    assert_eq!(Value::eval_ast(ast), Value::Int(6));

    let ast = AST::binop(
        AST::binop(AST::Number(1), OP::Mul, AST::Number(2)),
        OP::Add,
        AST::Number(3),
    );
    assert_eq!(Value::eval_ast(ast), Value::Int(5));

    let ast = AST::binop(
        AST::binop(AST::Number(1), OP::Mul, AST::Number(2)),
        OP::Minus,
        AST::Number(3),
    );
    assert_eq!(Value::eval_ast(ast), Value::Int(-1));

    let ast = AST::binop(
        AST::binop(AST::Number(1), OP::Mul, AST::Number(2)),
        OP::EqEq,
        AST::Number(2),
    );
    assert_eq!(Value::eval_ast(ast), Value::Bool(true));
}
