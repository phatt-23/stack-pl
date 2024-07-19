use super::Token;

static DEFAULT_VALUE: i64 = -255; 

#[derive(Debug, Copy, Clone)]
pub struct Operation {
    pub index: usize,
    pub token: Token,
    pub value: i64,
}

pub fn op_multiply(index: usize) -> Operation {
    Operation { index, token: Token::Multiply, value: DEFAULT_VALUE }
}

pub fn op_divide(index: usize) -> Operation {
    Operation { index, token: Token::Divide, value: DEFAULT_VALUE }
}

pub fn op_modulo(index: usize) -> Operation {
    Operation { index, token: Token::Modulo, value: DEFAULT_VALUE }
}

pub fn op_eq_gr(index: usize) -> Operation {
    Operation { index, token: Token::EqGr, value: DEFAULT_VALUE }
}

pub fn op_eq_le(index: usize) -> Operation {
    Operation { index, token: Token::EqLe, value: DEFAULT_VALUE }    
}

pub fn op_not(index: usize) -> Operation {
    Operation { index, token: Token::Not, value: DEFAULT_VALUE }    
}

pub fn op_dup(index: usize) -> Operation {
    Operation { index, token: Token::Dup, value: DEFAULT_VALUE }    
}

pub fn op_while(index: usize) -> Operation {
    Operation { index, token: Token::While, value: DEFAULT_VALUE }    
}

pub fn op_do(index: usize) -> Operation {
    Operation { index, token: Token::Do, value: DEFAULT_VALUE }    
}

pub fn op_else(index: usize) -> Operation {
    Operation { index, token: Token::Else, value: DEFAULT_VALUE }    
}

pub fn op_if(index: usize) -> Operation {
    Operation { index, token: Token::If, value: DEFAULT_VALUE }    
}

pub fn op_end(index: usize) -> Operation {
    Operation { index, token: Token::End, value: DEFAULT_VALUE }    
}

pub fn op_eq(index: usize) -> Operation {
    Operation { index, token: Token::Eq, value: DEFAULT_VALUE }
}

pub fn op_not_eq(index: usize) -> Operation {
    Operation { index, token: Token::NotEq, value: DEFAULT_VALUE }
}

pub fn op_le(index: usize) -> Operation {
    Operation { index, token: Token::Le, value: DEFAULT_VALUE }
}

pub fn op_gr(index: usize) -> Operation {
    Operation { index, token: Token::Gr, value: DEFAULT_VALUE }
}

pub fn op_plus(index: usize) -> Operation {
    Operation { index, token: Token::Plus, value: DEFAULT_VALUE }
}

pub fn op_minus(index: usize) -> Operation {
    Operation { index, token: Token::Minus, value: DEFAULT_VALUE }
}

pub fn op_dump(index: usize) -> Operation {
    Operation { index, token: Token::Dump, value: DEFAULT_VALUE }
}

pub fn op_push(index: usize, value: i64) -> Operation {
    Operation { index, token: Token::Push, value }
}