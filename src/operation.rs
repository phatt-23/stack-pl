use std::collections::HashMap;
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
    Macro,
    If(i32),
    Else(i32),
    Do(i32),
    End(i32),
    Include(String),
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

impl OperationKind {
    const JUMP_DEFAULT: i32 = -255;
    const INCLUDE_DEFAULT: &'static str = "NoIncludeFileProvided";

    pub fn from_str(s: &str) -> Option<Self> {
        return HashMap::from([
            // stack
            ("dup",     OperationKind::Duplicate),
            ("dup2",    OperationKind::Duplicate2),
            ("drop",    OperationKind::Drop),
            ("swap",    OperationKind::Swap),
            ("over",    OperationKind::Over),
            // io
            ("dump",    OperationKind::Dump),
            ("print",   OperationKind::PrintChar),
            // arithmetic
            ("+",       OperationKind::Add),
            ("add",     OperationKind::Add),
            ("-",       OperationKind::Subtract),
            ("sub",     OperationKind::Subtract),
            ("*",       OperationKind::Multiply),
            ("mul",     OperationKind::Multiply),
            ("/",       OperationKind::Divide),
            ("div",     OperationKind::Divide),
            ("%",       OperationKind::Modulo),
            ("mod",     OperationKind::Modulo),
            // logic
            ("=",       OperationKind::Equal),
            ("eq",      OperationKind::Equal),
            ("!=",      OperationKind::NotEqual),
            ("neq",     OperationKind::NotEqual),
            ("<",       OperationKind::Less),
            ("le",      OperationKind::Less),
            (">",       OperationKind::Greater),
            ("gr",      OperationKind::Greater),
            ("<=",      OperationKind::LessEqual),
            ("eql",     OperationKind::LessEqual),
            (">=",      OperationKind::GreaterEqual),
            ("egr",     OperationKind::GreaterEqual),
            ("!",       OperationKind::Not),
            ("not",     OperationKind::Not),
            // bitwise
            ("<<",      OperationKind::ShiftLeft),
            ("shl",     OperationKind::ShiftLeft),
            (">>",      OperationKind::ShiftRight),
            ("shr",     OperationKind::ShiftRight),
            ("&",       OperationKind::BitAnd),
            ("band",    OperationKind::BitAnd),
            ("|",       OperationKind::BitOr),
            ("bor",     OperationKind::BitOr),
            // block
            ("while",   OperationKind::While),
            ("if",      OperationKind::If(Self::JUMP_DEFAULT)),
            ("else",    OperationKind::Else(Self::JUMP_DEFAULT)),
            ("do",      OperationKind::Do(Self::JUMP_DEFAULT)),
            ("end",     OperationKind::End(Self::JUMP_DEFAULT)),
            // preprocessor
            ("macro",   OperationKind::Macro),
            ("include", OperationKind::Include(Self::INCLUDE_DEFAULT.to_string())),
            // memory
            ("mem",     OperationKind::MemoryPush),
            (",",       OperationKind::MemoryLoad),
            ("load",    OperationKind::MemoryLoad),
            (".",       OperationKind::MemoryStore),
            ("store",   OperationKind::MemoryStore),
            // syscall
            ("syscall1",    OperationKind::Syscall1),
            ("syscall2",    OperationKind::Syscall2),
            ("syscall3",    OperationKind::Syscall3),
            ("syscall4",    OperationKind::Syscall4),
            ("syscall5",    OperationKind::Syscall5),
            ("syscall6",    OperationKind::Syscall6),
        ]).get(s).cloned();
    }
}
