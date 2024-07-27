#[derive(PartialEq, Clone, Debug)]
pub enum IntrinsicType {
    // Stack
    Duplicate,
    Drop,
    Swap,
    Over,
    Rotate,
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
            // logic
            "=" => Some(Self::Equal),
            "!=" => Some(Self::NotEqual),
            "<" => Some(Self::Less),
            "<=" => Some(Self::LessEqual),
            ">" => Some(Self::Greater),
            ">=" => Some(Self::GreaterEqual),
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
