#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum Token {
    // stack manip
    Push,
    Dup,
    Dump,
    // binary arithmetic operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    // binary logic operators
    Eq,
    NotEq,
    Le,
    Gr,
    EqGr,
    EqLe,
    // unary logic operators
    Not,
    // block enclosing
    End,
    // block headers
    If,
    Else,
    While,
    Do,
}
