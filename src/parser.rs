use self::AST::*;
use crate::ast::{Member, AST, OP};
use crate::primitive::Type;
use crate::token::{KeywordKind, Token};
use std::collections::VecDeque;

pub struct Parser {
    index: usize,
    tokens: VecDeque<Token>,
}

macro_rules! expect {
    ($self: ident, $variant: ident, $value: expr) => {
        let t = $self.get();
        if let Token::$variant($value) = t {
            ()
        } else {
            panic!("Unexpected token! {:?}", t)
        }
    };
}

macro_rules! get {
    ($self: ident, $variant: ident) => {
        if let Token::$variant(value) = $self.get() {
            value
        } else {
            unreachable!()
        }
    };
}

macro_rules! def_parse_binop {
    ($name: ident, $one: ident, $another: ident, $next: ident) => {
        fn $name(&mut self) -> AST {
        let mut left = self.$next();
        loop {
            match self.peek() {
                Some(&Token::$one) | Some(&Token::$another) => (),
                _ => break
            }
            let op = self.get();
            let right = self.$next();
            left = AST::binop(left, OP::from_token(op), right);
        }
        left
    }
    };
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { index: 0, tokens }
    }

    fn term(&mut self) -> AST {
        let t = self.get();
        match t {
            Token::Number(n) => Number(n),
            Token::StrLiteral(s) => StrLiteral(s),
            _ => {
                dbg!(t);
                dbg!(&self.tokens);
                panic!();
            }
        }
    }

    def_parse_binop!(mul, Mul, Slash, term);
    def_parse_binop!(add, Add, Minus, mul);

    fn equal(&mut self) -> AST {
        let mut left = self.add();
        loop {
            if self.peek() != Some(&Token::EqEq) {
                break;
            }
            // TODO: Fix hard coding.
            self.get();
            let right = self.add();
            left = AST::binop(left, OP::EqEq, right);
        }
        left
    }

    fn expr(&mut self) -> AST {
        self.equal()
    }

    fn method_call(&mut self) -> AST {
        let table = get!(self, Ident);
        expect!(self, Symbol, '.');
        let name = get!(self, Ident);
        expect!(self, Symbol, '(');
        let args = {
            if let Some(&Token::Symbol(')')) = self.peek() {
                vec![]
            } else {
                let mut v = vec![];
                v.push(self.expr());
                while let Some(&Token::Symbol(',')) = self.peek() {
                    self.get();
                    v.push(self.expr());
                }
                v
            }
        };
        expect!(self, Symbol, ')');
        AST::MethodCall { table, name, args }
    }

    fn peek(&self) -> Option<&Token> {
        if let Some(t) = self.tokens.get(self.index) {
            Some(t)
        } else {
            None
        }
    }

    fn get(&mut self) -> Token {
        if let Some(t) = self.tokens.pop_front() {
            t
        } else {
            panic!();
        }
    }

    fn table_def(&mut self) -> AST {
        self.get();
        let name = get!(self, Ident);
        expect!(self, Symbol, '{');
        let members = if let Some(&Token::Symbol('}')) = self.peek() {
            vec![]
        } else {
            let mut v = vec![];
            loop {
                match self.peek() {
                    Some(Token::Ident(_)) => {
                        let field = get!(self, Ident);
                        expect!(self, Symbol, ':');
                        let typ = Type::from_token(self.get());
                        v.push(Member { typ, field });
                    }
                    Some(Token::Symbol(',')) => {
                        self.get();
                        continue;
                    }
                    Some(Token::Symbol('}')) => break,
                    _ => unreachable!(),
                }
            }
            v
        };
        expect!(self, Symbol, '}');
        AST::TableDef { name, members }
    }

    pub fn parse(&mut self) -> AST {
        match self.peek().unwrap() {
            Token::Number(_) => self.expr(),
            Token::Ident(_) => self.method_call(),
            Token::Keyword(KeywordKind::Table) => self.table_def(),
            _ => unimplemented!(),
        }
    }
}

#[allow(unused_imports)]
use crate::tokenizer::Tokenizer;

#[test]
fn new() {
    let tokens = Tokenizer::new("hogefuga").lex_all();
    assert_eq!(Parser::new(tokens).index, 0);
}

#[test]
fn term() {
    let s = "\"kuru\"";
    let tokens = Tokenizer::new(&s).lex_all();
    assert_eq!(Parser::new(tokens).term(), StrLiteral("kuru".to_string()));
}

#[test]
fn add() {
    let tokens = Tokenizer::new("1 + 2 + 3").lex_all();
    assert_eq!(
        Parser::new(tokens).add(),
        AST::binop(
            AST::binop(Number(1), OP::Add, Number(2)),
            OP::Add,
            Number(3)
        )
    );
}

#[test]
fn mul() {
    let tokens = Tokenizer::new("1 + 2 * 3").lex_all();
    assert_eq!(
        Parser::new(tokens).add(),
        AST::binop(
            Number(1),
            OP::Add,
            AST::binop(Number(2), OP::Mul, Number(3))
        )
    );

    let tokens = Tokenizer::new("1 + 2 / 3").lex_all();
    assert_eq!(
        Parser::new(tokens).add(),
        AST::binop(
            Number(1),
            OP::Add,
            AST::binop(Number(2), OP::Div, Number(3))
        )
    );
}

#[test]
fn equal() {
    let tokens = Tokenizer::new("4 == 2 + 3").lex_all();
    assert_eq!(
        Parser::new(tokens).equal(),
        AST::binop(
            Number(4),
            OP::EqEq,
            AST::binop(Number(2), OP::Add, Number(3))
        )
    );
}

#[test]
fn method_call() {
    let tokens = Tokenizer::new("User.select()").lex_all();
    assert_eq!(
        Parser::new(tokens).method_call(),
        AST::MethodCall {
            table: "User".to_string(),
            name: "select".to_string(),
            args: vec![]
        }
    );

    let tokens = Tokenizer::new("User.select(1)").lex_all();
    assert_eq!(
        Parser::new(tokens).method_call(),
        AST::MethodCall {
            table: "User".to_string(),
            name: "select".to_string(),
            args: vec![Number(1)]
        }
    );

    let tokens = Tokenizer::new("User.select(1+1, 2)").lex_all();
    assert_eq!(
        Parser::new(tokens).method_call(),
        AST::MethodCall {
            table: "User".to_string(),
            name: "select".to_string(),
            args: vec![AST::binop(Number(1), OP::Add, Number(1)), Number(2)]
        }
    );
}

#[test]
fn table_def() {
    let tokens = Tokenizer::new("Table NewUser {}").lex_all();
    assert_eq!(
        Parser::new(tokens).table_def(),
        AST::TableDef {
            name: "NewUser".to_string(),
            members: vec![]
        }
    );

    let tokens = Tokenizer::new("Table NewUser {id: int}").lex_all();
    assert_eq!(
        Parser::new(tokens).table_def(),
        AST::TableDef {
            name: "NewUser".to_string(),
            members: vec![Member {
                typ: Type::Int,
                field: "id".to_string(),
            }]
        }
    );

    let tokens = Tokenizer::new("Table NewUser {id: int, name: string,}").lex_all();
    assert_eq!(
        Parser::new(tokens).table_def(),
        AST::TableDef {
            name: "NewUser".to_string(),
            members: vec![
                Member {
                    typ: Type::Int,
                    field: "id".to_string(),
                },
                Member {
                    typ: Type::StrLiteral,
                    field: "name".to_string(),
                }
            ]
        }
    );

    let tokens = Tokenizer::new("Table NewUser {id: int, name: string}").lex_all();
    assert_eq!(
        Parser::new(tokens).table_def(),
        AST::TableDef {
            name: "NewUser".to_string(),
            members: vec![
                Member {
                    typ: Type::Int,
                    field: "id".to_string(),
                },
                Member {
                    typ: Type::StrLiteral,
                    field: "name".to_string(),
                }
            ]
        }
    );
}
