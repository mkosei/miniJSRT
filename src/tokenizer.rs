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

            if c.is_ascii_alphabetic() {
                let ident = self.read_ident();
                tokens.push(Token::Ident(ident));
            } else if c.is_ascii_digit() {
                let number = self.read_number();
                tokens.push(Token::Number(number));
            } else if c == '"' {
                let string = self.read_string();
                tokens.push(Token::String(string));
            } else {
                match c {
                    '(' => tokens.push(Token::LParen(c)),
                    ')' => tokens.push(Token::RPares(c)),
                    ',' => tokens.push(Token::Comma(c)),
                    _ => panic!("Unexpected char: {}", c),
                }
                self.advance();
            }
            tokens
        }

        fn read_ident(&mut self) -> String {
            let mut s = String::new();
            while let Some(c) = self.current() {
                if c.is_ascii_alphabetic() || c == '_' {
                    s.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            s
        }

        fn read_number(&mut self) -> i64 {
            let mut s = String::new();
            while let Some(c) = self.current() {
                if c.is_ascii_digit {
                    s.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            s.parse().unwrap();
        }

        fn read_string(&mut self) -> String {
            let mut s = String::new();
            while let Some(c) = self.current() {
                if c == '"' {
                    self.advance();
                    break;
                }
                s.push(c);
                self.advance
            }
            s
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
