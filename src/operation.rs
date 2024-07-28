// use crate::keyword::KeywordType;
use crate::location::Location;
use crate::intrinsic::IntrinsicType;

#[derive(Debug, PartialEq, Clone)]
pub enum OperationKind {
    // Push Types
    PushInt32(i32),
    PushInt64(i64),
    PushStr(String),
    PushChar(char),
    Intrinsic(IntrinsicType),
    // IO
    PrintInt64,
    PrintChar,
    // Keyword(KeywordType, Option<usize>),
    While,
    If(Option<usize>),
    Else(Option<usize>),
    Do(Option<usize>),
    End(Option<usize>),
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
            loc:     location,
            address: Self::DEFAULT_ADDRESS,
            kind:    op_kind,
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation {}\taddress: {:<3}\tkind: {:?}", self.loc, self.address, self.kind)
    }
}

impl OperationKind {
    pub fn from_str_builtin(s: &str) -> Option<Self> {
        match s {
            "print"  => Some(OperationKind::PrintInt64),
            "printc" => Some(OperationKind::PrintChar),
            _ => None
        }
    }
}
