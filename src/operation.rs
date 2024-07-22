
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OperationType {
    // PushTypes
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
    Macro,
    End,
    // Memory
    MemoryPush,
    MemoryLoad,
    MemoryStore,
    // Syscall
    Syscall1,
    Syscall2,
    Syscall3,
    Syscall4,
    Syscall5,
    Syscall6,
}

#[derive(Debug, Clone)]
pub enum OperationValue {
    Nothing,
    Int(i32),
    Str(String),
}

use std::collections::HashMap;

use crate::location::Location;

#[derive(Debug, Clone)]
pub struct Operation {
    pub loc: Location,
    pub address: usize,
    pub op_type: OperationType,
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
        write!(f, "Operation {}\taddress: {:<3}\tjump: {:<3}\top_type: {:<12}\tvalue: {:<10}", 
            self.loc, self.address, self.jump, op_type, value)
    }
}

impl OperationType {
    pub fn from_str(s: &str) -> Option<Self> {
        return HashMap::from([
            // stack
            ("dump",    OperationType::Dump),
            ("dup",     OperationType::Duplicate),
            ("dup2",    OperationType::Duplicate2),
            ("drop",    OperationType::Drop),
            ("swap",    OperationType::Swap),
            ("over",    OperationType::Over),
            // arithmetic
            ("+",       OperationType::Add),
            ("add",     OperationType::Add),
            ("-",       OperationType::Subtract),
            ("sub",     OperationType::Subtract),
            ("*",       OperationType::Multiply),
            ("mul",     OperationType::Multiply),
            ("/",       OperationType::Divide),
            ("div",     OperationType::Divide),
            ("%",       OperationType::Modulo),
            ("mod",     OperationType::Modulo),
            // logic
            ("=",       OperationType::Equal),
            ("eq",      OperationType::Equal),
            ("!=",      OperationType::NotEqual),
            ("neq",     OperationType::NotEqual),
            ("<",       OperationType::Less),
            ("le",      OperationType::Less),
            (">",       OperationType::Greater),
            ("gr",      OperationType::Greater),
            ("<=",      OperationType::LessEqual),
            ("eql",     OperationType::LessEqual),
            (">=",      OperationType::GreaterEqual),
            ("egr",     OperationType::GreaterEqual),
            ("!",       OperationType::Not),
            ("not",     OperationType::Not),
            // bitwise
            ("<<",      OperationType::ShiftLeft),
            ("shl",     OperationType::ShiftLeft),
            (">>",      OperationType::ShiftRight),
            ("shr",     OperationType::ShiftRight),
            ("&",       OperationType::BitAnd),
            ("band",    OperationType::BitAnd),
            ("|",       OperationType::BitOr),
            ("bor",     OperationType::BitOr),
            // block
            ("if",      OperationType::If),
            ("else",    OperationType::Else),
            ("while",   OperationType::While),
            ("do",      OperationType::Do),
            ("macro",   OperationType::Macro),
            ("end",     OperationType::End),
            // memory
            ("mem",     OperationType::MemoryPush),
            (",",       OperationType::MemoryLoad),
            ("load",    OperationType::MemoryLoad),
            (".",       OperationType::MemoryStore),
            ("store",   OperationType::MemoryStore),
            // syscall
            ("syscall1",    OperationType::Syscall1),
            ("syscall2",    OperationType::Syscall2),
            ("syscall3",    OperationType::Syscall3),
            ("syscall4",    OperationType::Syscall4),
            ("syscall5",    OperationType::Syscall5),
            ("syscall6",    OperationType::Syscall6),
        ]).get(s).cloned();
    }
    pub fn from_str_simple(s: &str) -> Option<Self> {
        return HashMap::from([
            // stack
            ("dump",    OperationType::Dump),
            ("dup",     OperationType::Duplicate),
            ("dup2",    OperationType::Duplicate2),
            ("drop",    OperationType::Drop),
            ("swap",    OperationType::Swap),
            ("over",    OperationType::Over),
            // arithmetic
            ("+",       OperationType::Add),
            ("add",     OperationType::Add),
            ("-",       OperationType::Subtract),
            ("sub",     OperationType::Subtract),
            ("*",       OperationType::Multiply),
            ("mul",     OperationType::Multiply),
            ("/",       OperationType::Divide),
            ("div",     OperationType::Divide),
            ("%",       OperationType::Modulo),
            ("mod",     OperationType::Modulo),
            // logic
            ("=",       OperationType::Equal),
            ("eq",      OperationType::Equal),
            ("!=",      OperationType::NotEqual),
            ("neq",     OperationType::NotEqual),
            ("<",       OperationType::Less),
            ("le",      OperationType::Less),
            (">",       OperationType::Greater),
            ("gr",      OperationType::Greater),
            ("<=",      OperationType::LessEqual),
            ("eql",     OperationType::LessEqual),
            (">=",      OperationType::GreaterEqual),
            ("egr",     OperationType::GreaterEqual),
            ("!",       OperationType::Not),
            ("not",     OperationType::Not),
            // bitwise
            ("<<",      OperationType::ShiftLeft),
            ("shl",     OperationType::ShiftLeft),
            (">>",      OperationType::ShiftRight),
            ("shr",     OperationType::ShiftRight),
            ("&",       OperationType::BitAnd),
            ("band",    OperationType::BitAnd),
            ("|",       OperationType::BitOr),
            ("bor",     OperationType::BitOr),
            // memory
            ("mem",     OperationType::MemoryPush),
            (",",       OperationType::MemoryLoad),
            ("load",    OperationType::MemoryLoad),
            (".",       OperationType::MemoryStore),
            ("store",   OperationType::MemoryStore),
            // syscall
            ("syscall1",    OperationType::Syscall1),
            ("syscall2",    OperationType::Syscall2),
            ("syscall3",    OperationType::Syscall3),
            ("syscall4",    OperationType::Syscall4),
            ("syscall5",    OperationType::Syscall5),
            ("syscall6",    OperationType::Syscall6),
        ]).get(s).cloned();
    }
    pub fn from_str_block(s: &str) -> Option<Self> {
        return HashMap::from([
            // block
            ("if",      OperationType::If),
            ("else",    OperationType::Else),
            ("while",   OperationType::While),
            ("do",      OperationType::Do),
            ("macro",   OperationType::Macro),
            ("end",     OperationType::End),
        ]).get(s).cloned();
    }
}