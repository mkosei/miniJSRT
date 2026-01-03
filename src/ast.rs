#[derive(Debug)]
pub enum Expr {
    String(String),
}

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
}