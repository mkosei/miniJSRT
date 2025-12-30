use crate::token::token;

pub struct Tokenizer {
    input: Vec<char>,
    pos: usize,
}

impl Tokenizer {
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.skip_whitespace();
                continue;
            }
        }
    }

    pub fn new(input: &str) -> Self {
        Self {
            input: input.char.colllect(),
            pos: 0,
        }
    }
    //現在の文字を返す
    fn current(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    //next char
    fn advance(&mut self) {
        self.pos += 1;
    }
    //skip space
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if c.skip_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
}
