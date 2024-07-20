
#[derive(Debug, PartialEq, Clone, Copy)]
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
    pub loc: Location,
    pub op_type: OperationType,
    pub address: usize,
    pub value: i32,
    pub jump: i32,
}


impl Operation {
    const DEFAULT_VALUE: i32 = -255;
    const DEFAULT_JUMP: i32 = -255;
    pub fn new(address: usize, op_type: OperationType, loc: &Location) -> Self {
        Self { 
            loc: loc.clone(),
            op_type,
            address,
            jump: Operation::DEFAULT_JUMP,
            value: Operation::DEFAULT_VALUE, 
        }
    }
    pub fn new_with_value(address: usize, op_type: OperationType, value: i32, loc: &Location) -> Self {
        Self { 
            loc: loc.clone(),
            op_type, 
            address,
            jump: Operation::DEFAULT_JUMP,
            value,
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_type = format!("{:?}", self.op_type);
        write!(f, "Operation \t{} \top_type: {:<12} \tvalue: {:<5?} \taddress: {:<3} \tjump {:<3}", 
            self.loc, op_type, self.value, self.address, self.jump)
    }
}