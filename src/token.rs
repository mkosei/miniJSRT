#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(i64),
    String(String),

    LParen,
    RParen,
    Comma,
}
