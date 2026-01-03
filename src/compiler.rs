use crate::ast::{Expr, Stmt};
use crate::chunk::{Chunk};
use crate::opcode::Op;
use crate::value::Value;

pub struct Compiler{
    pub chunk: Chunk,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
        }
    }

    // ステートメント単位のコンパイル
    pub fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                self.compile_expr(expr);
                self.chunk.emit(Op::Print);
            }
        }
    }
    //式をコンパイル
    pub fn compile_expr(&mut self , expr: Expr) {
        match expr {
            Expr::String(s) => {
                let id = self.chunk.add_constant(Value::String(s));
                self.chunk.emit(Op::LoadConst(id));
            }
        }
    }

}