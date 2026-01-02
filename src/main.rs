mod token;
mod tokenizer;
mod parse;

use tokenizer::Tokenizer;
use parse::Parser;
use parse::Expr;

fn main() {
   let code = r#"print("hello")"#;
   let mut t = Tokenizer::new(code);
   let tokens = t.tokenize();

   let mut p = Parser::new(tokens);
   let exprs = p.parse();

   for expr in exprs {
       match expr {
           Expr::PrintString(s) => print!("{}", s),
           Expr::PrintNumber(n) => print!("{}", n),
       }
   }
   println!();
}
