use crate::token::Token;

pub enum Expr {
    PrintString(String),
    PrintNumber(i64),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, token: &Token) {
        if self.current() != Some(token) {
            panic!("Expected {:?}, got {:?}", token, self.current());
        }
        self.advance();
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut exprs = vec![];

        while let Some(tok) = self.current() {
            match tok {
                Token::Ident(s) if s == "print" => {
                    self.advance();
                    self.expect(&Token::LParen);

                    if let Some(Token::String(val)) = self.current() {
                        exprs.push(Expr::PrintString(val.clone()));
                        self.advance();
                    } else if let Some(Token::Number(val)) = self.current() {
                        exprs.push(Expr::PrintNumber(*val));
                        self.advance();
                    } else {
                        panic!("Unexpected token after print(");
                    }

                    self.expect(&Token::RParen);
                }
                _ => panic!("Unexpected token: {:?}", tok),
            }
        }

        exprs
    }
}
