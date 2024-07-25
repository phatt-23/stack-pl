use crate::location::Location;

#[derive(Debug, PartialEq, Clone)]
pub enum OperationKind {
    // PushTypes
    PushInt(i32),
    PushStr(String),
    PushChar(char),
    // Stack
    Duplicate,
    Duplicate2,
    Drop,
    Swap,
    Over,
    // IO
    Dump,
    PrintChar,
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
    // Keyword
    While,
    If(i32),
    Else(i32),
    Do(i32),
    End(i32),
    // Macro,            // <--- now as a keyword, recognized by the tokenizer
    // Include(String),  // <---
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
            // stack
            "dup"         => Some(OperationKind::Duplicate),
            "dup2"        => Some(OperationKind::Duplicate2),
            "drop"        => Some(OperationKind::Drop),
            "swap"        => Some(OperationKind::Swap),
            "over"        => Some(OperationKind::Over),
            // io
            "dump"        => Some(OperationKind::Dump),
            "print"       => Some(OperationKind::PrintChar),
            // arithmetic
            "+" | "add"   => Some(OperationKind::Add),
            "-" | "sub"   => Some(OperationKind::Subtract),
            "*" | "mul"   => Some(OperationKind::Multiply),
            "/" | "div"   => Some(OperationKind::Divide),
            "%" | "mod"   => Some(OperationKind::Modulo),
            // logic
            "="  | "eq"   => Some(OperationKind::Equal),
            "!=" | "neq"  => Some(OperationKind::NotEqual),
            "<"  | "le"   => Some(OperationKind::Less),
            ">"  | "gr"   => Some(OperationKind::Greater),
            "<=" | "eql"  => Some(OperationKind::LessEqual),
            ">=" | "egr"  => Some(OperationKind::GreaterEqual),
            "!"  | "not"  => Some(OperationKind::Not),
            // bitwise
            "<<" | "shl"  => Some(OperationKind::ShiftLeft),
            ">>" | "shr"  => Some(OperationKind::ShiftRight),
            "&"  | "band" => Some(OperationKind::BitAnd),
            "|"  | "bor"  => Some(OperationKind::BitOr),
            // memory
            "mem"         => Some(OperationKind::MemoryPush),
            "load" | ","  => Some(OperationKind::MemoryLoad),
            "store"| "."  => Some(OperationKind::MemoryStore),
            // syscall
            "syscall1"    => Some(OperationKind::Syscall1),
            "syscall2"    => Some(OperationKind::Syscall2),
            "syscall3"    => Some(OperationKind::Syscall3),
            "syscall4"    => Some(OperationKind::Syscall4),
            "syscall5"    => Some(OperationKind::Syscall5),
            "syscall6"    => Some(OperationKind::Syscall6),
            _ => None
        }
    }

    // pub fn from_str_keyword(keyword: KeywordType) -> Option<Self> {
    //     match keyword {
    //         KeywordType::If => Some(OperationKind::If(JUMP_DEFAULT)),
    //         KeywordType::End => Some(OperationKind::End(JUMP_DEFAULT)),
    //         KeywordType::Else => Some(OperationKind::Else(JUMP_DEFAULT)),
    //         KeywordType::While => Some(OperationKind::While),
    //         KeywordType::Do => Some(OperationKind::Do(JUMP_DEFAULT)),
    //         _ => None
    //     }
    // }
}
