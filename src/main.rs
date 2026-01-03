mod ast;
mod compiler;
mod chunk;
mod value;
mod opcode;

use crate::ast::{Expr,Stmt};
use crate::compiler::Compiler;

fn main() {
    let ast = vec![
        Stmt::Print(Expr::String("hello".to_string())),
        Stmt::Print(Expr::String("world".to_string())),
    ];

    let mut compiler = Compiler::new();

    for stmt in ast {
        compiler.compile_stmt(stmt);
    }

    compiler.chunk.emit(crate::opcode::Op::Halt);
    println!("Bytecode:");

    for (i, val) in compiler.chunk.constants.iter().enumerate() {
        println!("{}: {:?}", i, val);
    }

}

// mod parse;
// mod token;
// mod tokenizer;

// use parse::Expr;
// use parse::Parser;
// use tokenizer::Tokenizer;

// use std::env;
// use std::fs;

// fn main() {
//     let path = env::args().nth(1).expect("No input file specified");
//     let code = fs::read_to_string(path).expect("Failed to read the file");
//     let mut t = Tokenizer::new(&code);
//     let tokens = t.tokenize();

//     let mut p = Parser::new(tokens);
//     let exprs = p.parse();

//     for expr in exprs {
//         match expr {
//             Expr::PrintString(s) => print!("{}", s),
//             Expr::PrintNumber(n) => print!("{}", n),
//         }
//     }
//     println!();
// }

//ASTを捨ててBytecodeコンパイラを作成することで分岐の深さや、CPU負荷などの問題を解決

