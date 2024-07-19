#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum Token {
    Push,
    Dump,
    Plus,
    Minus,
    Eq,
    Le,
    Gr,
    End,
    If,
    Else,
}