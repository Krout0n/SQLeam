use crate::token::Token;

pub struct Tokenizer<'a> {
    src: &'a str,
    index: usize,
    ch: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            index: 0,
            ch: None,
        }
    }

    pub fn lex(&mut self) -> Option<Token> {
        self.read_char();
        match self.ch {
            // Skip blank chars.
            Some(' ') | Some('\n') => self.lex(),

            // Ident or Keyword?
            Some('a'...'z') | Some('A'...'Z') => {
                let mut buffer = String::new();
                while let Some('a'...'z') | Some('A'...'Z') = self.ch {
                    buffer.push(self.ch.unwrap());
                    self.read_char();
                }
                self.backtrack();
                Some(Token::Ident(buffer))
            }

            // Number
            Some('0'...'9') => {
                let mut buffer = String::new();
                while let Some('0'...'9') = self.ch {
                    buffer.push(self.ch.unwrap());
                    self.read_char();
                }
                self.backtrack();
                Some(Token::Number(buffer.parse().unwrap()))
            }

            // Only Symbol?
            Some(ch) => match ch {
                '+' | '-' | '*' | '/' | '(' | ')' | '.' | ';' => Some(Token::Symbol(ch)),
                _ => unimplemented!(),
            },
            _ => None,
        }
    }

    pub fn lex_all(&mut self) -> Vec<Token> {
        let mut result = vec![];
        while let Some(token) = self.lex() {
            result.push(token);
        }
        result
    }

    fn peek(&self) -> Option<char> {
        self.src.chars().nth(self.index)
    }

    fn read_char(&mut self) {
        self.ch = self.src.chars().nth(self.index);
        self.index += 1;
    }

    fn backtrack(&mut self) {
        self.index -= 1;
        self.ch = self.src.chars().nth(self.index);
    }
}

mod tests {
    use super::*;
    #[test]
    fn new() {
        let t = Tokenizer::new("hello");
        assert_eq!(t.src, "hello");
    }

    #[test]
    fn lex() {
        let mut t = Tokenizer::new("hello");
        assert_eq!(t.lex(), Some(Token::Ident("hello".to_string())));

        let mut t = Tokenizer::new("Knium is godlike!");
        assert_eq!(t.lex(), Some(Token::Ident("Knium".to_string())));

        let mut t = Tokenizer::new("42");
        assert_eq!(t.lex(), Some(Token::Number(42)));

        let mut t = Tokenizer::new("+");
        assert_eq!(t.lex(), Some(Token::Symbol('+')));
    }

    #[test]
    fn lex_all() {
        let mut t = Tokenizer::new("42+15");
        assert_eq!(
            t.lex_all(),
            vec![Token::Number(42), Token::Symbol('+'), Token::Number(15),]
        );

        let input = "User.select();";
        let mut t = Tokenizer::new(input);
        assert_eq!(
            t.lex_all(),
            vec![
                Token::Ident("User".to_string()),
                Token::Symbol('.'),
                Token::Ident("select".to_string()),
                Token::Symbol('('),
                Token::Symbol(')'),
                Token::Symbol(';')
            ]
        );

        let mut t = Tokenizer::new("42      + 15 \n + 3");
        assert_eq!(
            t.lex_all(),
            vec![
                Token::Number(42),
                Token::Symbol('+'),
                Token::Number(15),
                Token::Symbol('+'),
                Token::Number(3)
            ]
        );
    }
}
