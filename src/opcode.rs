#[derive(Debug, Clone, Copy)]
pub enum Op {
    LoadConst(usize), // 定数をロードする命令
    Print,   // Print命令
    Halt,    // プログラムの終了
}