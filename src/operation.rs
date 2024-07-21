
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OperationType {
    
    PushInt,
    PushStr,
    // Stack
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

#[derive(Debug, Clone)]
pub enum OperationValue {
    Nothing,
    Int(i32),
    Str(String),
}

use super::location::Location;

#[derive(Debug, Clone)]
pub struct Operation {
    pub loc: Location,
    pub op_type: OperationType,
    pub address: usize,
    pub value: OperationValue,
    pub jump: i32,
}

impl Operation {
    const DEFAULT_JUMP: i32 = -255;
    pub fn new_value_none(address: usize, op_type: OperationType, loc: &Location) -> Self {
        Self {
            loc: loc.clone(),
            op_type,
            address,
            jump: Operation::DEFAULT_JUMP,
            value: OperationValue::Nothing, 
        }
    }
    pub fn new_value_int(address: usize, op_type: OperationType, value: i32, loc: &Location) -> Self {
        Self { 
            loc: loc.clone(),
            op_type, 
            address,
            jump: Operation::DEFAULT_JUMP,
            value: OperationValue::Int(value),
        }
    }
    pub fn new_value_str(address: usize, op_type: OperationType, value: &String, loc: &Location) -> Self {
        Self { 
            loc: loc.clone(),
            op_type, 
            address,
            jump: Operation::DEFAULT_JUMP,
            value: OperationValue::Str((*value).clone()),
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_type = format!("{:?}", self.op_type);
        let value = format!("{:?}", self.value);
        write!(f, "Operation \t{} \taddress: {:<3} \tjump {:<3} \top_type: {:<12} \tvalue: {:<10}", 
            self.loc, self.address, self.jump, op_type, value)
    }
}
