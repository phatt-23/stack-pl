use crate::location::Location;
use crate::intrinsic::IntrinsicType;

#[derive(Debug, PartialEq, Clone)]
pub enum OperationKind {
    // PushTypes
    PushInt(i32),
    PushStr(String),
    PushChar(char),
    // Stack
    // Arithmetic
    // Bitwise
    // Logic
    // Memory
    // Syscall
    Intrinsic(IntrinsicType),
    // IO
    Dump,
    PrintChar,
    While,
    // Keyword
    // Macro,            // <--- now as a keyword, recognized by the tokenizer
    // Include(String),  // <---
    If(i32),
    Else(i32),
    Do(i32),
    End(i32),
}



#[derive(Debug, Clone)]
pub struct Operation {
    pub loc: Location,
    pub address: usize,
    pub kind: OperationKind,
}

impl Operation {
    const DEFAULT_ADDRESS: usize = usize::MAX;
    pub fn new(op_kind: OperationKind, location: Location) -> Self {
        Self {
            loc:        location,
            address:    Self::DEFAULT_ADDRESS,
            kind:       op_kind,
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation {}\taddress: {:<3}\tkind: {:?}", self.loc, self.address, self.kind)
    }
}

pub const JUMP_DEFAULT: i32 = -255;

impl OperationKind {
    pub fn from_str_builtin(s: &str) -> Option<Self> {
        match s {
            "dump"        => Some(OperationKind::Dump),
            "print"       => Some(OperationKind::PrintChar),
            _ => None
        }
    }
}
