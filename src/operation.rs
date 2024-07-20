
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum Token {
    // Stack
    Push,
    Dup,
    Dup2,
    Drop,
    Swap,
    Over,
    // IO
    Dump,
    // Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    // Bitwise
    BitAnd,
    BitOr,
    ShiftRight,
    ShiftLeft,
    // Logic
    Eq,
    NotEq,
    Le,
    Gr,
    EqGr,
    EqLe,
    Not,
    // Block
    If,
    Else,
    While,
    Do,
    End, // enclosing
    // Memory
    Memory,
    Load,
    Store,
    // Syscall
    Syscall3,
    Syscall1,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub index: usize,
    pub token: Token,
    pub value: i64,
    
    pub file: String,
    pub row: usize,
    pub col: usize,
}
static DEFAULT_VALUE: i64 = -255; 

pub fn op_shift_left(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::ShiftLeft, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_shift_right(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::ShiftRight, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_bit_and(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::BitAnd, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_bit_or(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::BitOr, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_syscall_1(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Syscall1, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_syscall_3(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Syscall3, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_store(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Store, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_load(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Load, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_memory(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Memory, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_multiply(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Multiply, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_divide(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Divide, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_modulo(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Modulo, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_eq_gr(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::EqGr, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_eq_le(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::EqLe, value: DEFAULT_VALUE, file: file.to_string(), row, col }    
}

pub fn op_not(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Not, value: DEFAULT_VALUE, file: file.to_string(), row, col }    
}

pub fn op_drop(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Drop, value: DEFAULT_VALUE, file: file.to_string(), row, col }    
}

pub fn op_dup(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Dup, value: DEFAULT_VALUE, file: file.to_string(), row, col }    
}

pub fn op_dup_2(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Dup2, value: DEFAULT_VALUE, file: file.to_string(), row, col }    
}

pub fn op_swap(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Swap, value: DEFAULT_VALUE, file: file.to_string(), row, col }    
}

pub fn op_over(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Over, value: DEFAULT_VALUE, file: file.to_string(), row, col }    
}

pub fn op_while(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::While, value: DEFAULT_VALUE, file: file.to_string(), row, col }    
}

pub fn op_do(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Do, value: DEFAULT_VALUE, file: file.to_string(), row, col }    
}

pub fn op_else(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Else, value: DEFAULT_VALUE, file: file.to_string(), row, col }    
}

pub fn op_if(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::If, value: DEFAULT_VALUE, file: file.to_string(), row, col }    
}

pub fn op_end(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::End, value: DEFAULT_VALUE, file: file.to_string(), row, col }    
}

pub fn op_eq(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Eq, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_not_eq(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::NotEq, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_le(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Le, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_gr(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Gr, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_plus(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Plus, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_minus(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Minus, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_dump(index: usize, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Dump, value: DEFAULT_VALUE, file: file.to_string(), row, col }
}

pub fn op_push(index: usize, value: i64, file: &String, row: usize, col: usize) -> Operation {
    Operation { index, token: Token::Push, value, file: file.to_string(), row, col }
}