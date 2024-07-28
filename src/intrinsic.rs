#[derive(PartialEq, Clone, Debug)]
pub enum IntrinsicType {
    //Stack Maipulation
    Duplicate,
    Drop,
    Swap,
    Over,
    Rotate,
    //Arithmetic Operations
    Add,
    Subtract,
    Multiply,
    DivMod,
    //Bitwise Operations
    BitNegate,
    BitAnd,
    BitOr,
    ShiftRight,
    ShiftLeft,
    //Comparison and Equality
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Not,
    //Memory Operations
    MemPush,
    MemLoad8,
    MemStore8,
    MemLoad16,
    MemStore16,
    MemLoad32,
    MemStore32,
    MemLoad64,
    MemStore64,
    //Syscall
    Syscall1,
    Syscall2,
    Syscall3,
    Syscall4,
    Syscall5,
    Syscall6,
}

impl IntrinsicType {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            // stack
            "dup" => Some(Self::Duplicate),
            "drop" => Some(Self::Drop),
            "swap" => Some(Self::Swap),
            "over" => Some(Self::Over),
            "rot" => Some(Self::Rotate),
            // arithmetic
            "+" => Some(Self::Add),
            "-" => Some(Self::Subtract),
            "*" => Some(Self::Multiply),
            "divmod" => Some(Self::DivMod),
            // bit
            "neg" => Some(Self::BitNegate),
            "and" => Some(Self::BitAnd),
            "or" => Some(Self::BitOr),
            "shr" => Some(Self::ShiftRight),
            "shl" => Some(Self::ShiftLeft),
            // comparison
            "=" => Some(Self::Equal),
            "!=" => Some(Self::NotEqual),
            "<" => Some(Self::Less),
            "<=" => Some(Self::LessEqual),
            ">" => Some(Self::Greater),
            ">=" => Some(Self::GreaterEqual),
            "not" => Some(Self::Not),
            // memory
            "mem" => Some(Self::MemPush),
            "*8" => Some(Self::MemLoad8),
            "!8" => Some(Self::MemStore8),
            "*16" => Some(Self::MemLoad16),
            "!16" => Some(Self::MemStore16),
            "*32" => Some(Self::MemLoad32),
            "!32" => Some(Self::MemStore32),
            "*64" => Some(Self::MemLoad64),
            "!64" => Some(Self::MemStore64),
            // system
            "syscall1" => Some(Self::Syscall1),
            "syscall2" => Some(Self::Syscall2),
            "syscall3" => Some(Self::Syscall3),
            "syscall4" => Some(Self::Syscall4),
            "syscall5" => Some(Self::Syscall5),
            "syscall6" => Some(Self::Syscall6),
            _ => None
        }
    }
}
