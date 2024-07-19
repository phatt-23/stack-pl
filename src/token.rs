#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum Token {
    Push,
    Dup,
    Dump,

    Plus,
    Minus,
    
    Eq,
    Le,
    Gr,
    EqGr,
    EqLe,
    Not,
    
    End,
    
    If,
    Else,
    
    While,
    Do,
    
}