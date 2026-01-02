use crate::token::Token;

pub struct Tokenizer {
    input: Vec<char>,
    pos: usize,
}

impl Tokenizer {

    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
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
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            // 識別子
            if c.is_ascii_alphabetic() {
                let ident = self.read_ident();
                tokens.push(Token::Ident(ident));
            } 
            // 数字
            else if c.is_ascii_digit() {
                let number = self.read_number();
                tokens.push(Token::Number(number));
            } 
            // 文字列
            else if c == '"' {
                let string = self.read_string();
                tokens.push(Token::String(string));
            } 
            // 記号
            else {
                match c {
                    '(' => tokens.push(Token::LParen),
                    ')' => tokens.push(Token::RParen),
                    ',' => tokens.push(Token::Comma),
                    _ => panic!("Unexpected char: {}", c),
                }
                self.advance();
            }
        }

        tokens
    }

    fn read_ident(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.current() {
            if c.is_ascii_alphanumeric() || c == '_' {
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
            if c.is_ascii_digit() {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        s.parse().unwrap()
    }

    fn read_string(&mut self) -> String {
        self.advance(); // 開始の " をスキップ
        let mut s = String::new();
        while let Some(c) = self.current() {
            if c == '"' {
                self.advance(); // 終了の " をスキップ
                break;
            }
            s.push(c);
            self.advance();
        }
        s
    }
}


