use self::AST::*;
use crate::ast::AST;
use crate::token::Token;
use crate::tokenizer::Tokenizer;
use std::collections::VecDeque;

pub struct Parser {
    index: usize,
    tokens: VecDeque<Token>,
}

macro_rules! binop {
    ($left: expr, $op: expr, $right: expr) => {
        BinOP(Box::new($left), $op, Box::new($right))
    };
}

impl Parser {
    fn new(tokens: VecDeque<Token>) -> Self {
        Self { index: 0, tokens }
    }

    fn term(&mut self) -> AST {
        match self.get() {
            Token::Number(n) => Number(n),
            _ => unimplemented!(),
        }
    }

    fn add(&mut self) -> AST {
        let mut left = self.term();
        loop {
            let peeked = self.peek();
            if peeked != Some(&Token::Symbol('+')) && peeked != Some(&Token::Symbol('-')) {
                break;
            }
            let op = if let Token::Symbol(ch) = self.get() {
                ch
            } else {
                unreachable!()
            };
            let right = self.term();
            left = binop!(left, op, right);
        }
        left
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
}

#[test]

fn new() {
    let tokens = Tokenizer::new("hogefuga").lex_all();
    assert_eq!(Parser::new(tokens).index, 0);
}

#[test]
fn add() {
    let tokens = Tokenizer::new("1 + 2 + 3").lex_all();
    assert_eq!(
        Parser::new(tokens).add(),
        binop!(binop!(Number(1), '+', Number(2)), '+', Number(3))
    );
}
