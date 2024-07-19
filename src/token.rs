#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum Token {
    // Stack manip
    Push,
    Dup,
    // IO
    Dump,
    // Binary arithmetic operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    // Binary logic operators
    Eq,
    NotEq,
    Le,
    Gr,
    EqGr,
    EqLe,
    // Unary logic operators
    Not,
    // Block enclosing
    End,
    // Block headers
    If,
    Else,
    While,
    Do,
    // Memory operators
    Memory,
    Load,
    Store,
    // Syscall
    Syscall3,
    Syscall1,
}
