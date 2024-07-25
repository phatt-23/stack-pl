#[derive(PartialEq, Clone, Debug)]
pub enum IntrinsicType {
    // Stack
    Duplicate,
    Drop,
    Swap,
    Over,
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    DivMod,
    // Bitwise
    BitNegate,
    BitAnd,
    BitOr,
    ShiftRight,
    ShiftLeft,
    // Logic
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Not,
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

impl IntrinsicType {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            // stack
            "dup" => Some(Self::Duplicate),
            "drop" => Some(Self::Drop),
            "swap" => Some(Self::Swap),
            "over" => Some(Self::Over),
            // arithmetic
            "add" => Some(Self::Add),
            "sub" => Some(Self::Subtract),
            "mul" => Some(Self::Multiply),
            "divmod" => Some(Self::DivMod),
            // bit
            "neg" => Some(Self::BitNegate),
            "band" => Some(Self::BitAnd),
            "bor" => Some(Self::BitOr),
            "shr" => Some(Self::ShiftRight),
            "shl" => Some(Self::ShiftLeft),
            // logic
            "eq" => Some(Self::Equal),
            "neq" => Some(Self::NotEqual),
            "le" => Some(Self::Less),
            "leeq" => Some(Self::LessEqual),
            "gr" => Some(Self::Greater),
            "greq" => Some(Self::GreaterEqual),
            "not" => Some(Self::Not),
            // memory
            "memp" => Some(Self::MemoryPush),
            "meml" => Some(Self::MemoryLoad),
            "mems" => Some(Self::MemoryStore),
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
