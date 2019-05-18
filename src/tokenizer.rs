use crate::token::Token;

pub struct Tokenizer<'a> {
    src: &'a str,
    index: usize,
    peeked: usize,
    result: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            index: 0,
            peeked: 0,
            result: vec![],
        }
    }
}

mod tests {
    use super::Tokenizer;
    #[test]
    fn new() {
        let t = Tokenizer::new("hello");
        assert_eq!(t.src, "hello");
    }
}
