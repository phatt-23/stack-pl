
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum OperationType {
    // Stack
    Push,
    Duplicate,
    Duplicate2,
    Drop,
    Swap,
    Over,
    // IO
    Dump,
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    // Bitwise
    BitAnd,
    BitOr,
    ShiftRight,
    ShiftLeft,
    // Logic
    Equal,
    NotEqual,
    Less,
    Greater,
    GreaterEqual,
    LessEqual,
    Not,
    // Block
    If,
    Else,
    While,
    Do,
    End, // enclosing
    // Memory
    MemoryPush,
    MemoryLoad,
    MemoryStore,
    // Syscall
    Syscall3,
    Syscall1,
}

use super::location::Location;

#[derive(Debug, Clone)]
pub struct Operation {
    pub index: usize,
    pub op_type: OperationType,
    pub value: i64,
    
    pub loc: Location,
}
static DEFAULT_VALUE: i64 = -255; 

impl Operation {
    pub fn new(token: OperationType, index: usize, loc: &Location) -> Self {
        Self { 
            index, 
            op_type: 
            token, 
            value: DEFAULT_VALUE, 
            loc: loc.clone()
        }
    }
    pub fn new_with_value(token: OperationType, value: i64, index: usize, loc: &Location) -> Self {
        Self { 
            index, 
            op_type: 
            token, 
            value, 
            loc: loc.clone()
        }
    }
}

