#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(string),
    Number(i64),
    String(String),

    LParen,
    RParen,
    Comma,
}
