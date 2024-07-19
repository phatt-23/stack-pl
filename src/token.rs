#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum Token {
    // Stack
    Push,
    Dup,
    Dup2,
    Drop,
    // IO
    Dump,
    // Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    // Bitwise
    BitAnd,
    BitOr,
    ShiftRight,
    ShiftLeft,
    // Logic
    Eq,
    NotEq,
    Le,
    Gr,
    EqGr,
    EqLe,
    Not,
    // Block
    If,
    Else,
    While,
    Do,
    End, // enclosing
    // Memory
    Memory,
    Load,
    Store,
    // Syscall
    Syscall3,
    Syscall1,
}
