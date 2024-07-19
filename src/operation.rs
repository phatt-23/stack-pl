use super::Token;

#[derive(Debug, Copy, Clone)]
pub struct Operation {
    pub index: usize,
    pub token: Token,
    pub value: i64,
}

pub fn op_eqgr(index: usize) -> Operation {
    Operation { token: Token::EqGr, value: -1, index }    
}

pub fn op_eqle(index: usize) -> Operation {
    Operation { token: Token::EqLe, value: -1, index }    
}

pub fn op_not(index: usize) -> Operation {
    Operation { token: Token::Not, value: -1, index }    
}

pub fn op_dup(index: usize) -> Operation {
    Operation { token: Token::Dup, value: -1, index }    
}

pub fn op_while(index: usize) -> Operation {
    Operation { token: Token::While, value: -1, index }    
}

pub fn op_do(index: usize) -> Operation {
    Operation { token: Token::Do, value: -1, index }    
}

pub fn op_else(index: usize) -> Operation {
    Operation { token: Token::Else, value: -1, index }    
}

pub fn op_if(index: usize) -> Operation {
    Operation { token: Token::If, value: -1, index }    
}

pub fn op_end(index: usize) -> Operation {
    Operation { token: Token::End, value: -1, index }    
}

pub fn op_eq(index: usize) -> Operation {
    Operation { token: Token::Eq, value: -1, index }
}

pub fn op_le(index: usize) -> Operation {
    Operation { token: Token::Le, value: -1, index }
}

pub fn op_gr(index: usize) -> Operation {
    Operation { token: Token::Gr, value: -1, index }
}

pub fn op_plus(index: usize) -> Operation {
    Operation { token: Token::Plus, value: -1, index }
}

pub fn op_minus(index: usize) -> Operation {
    Operation { token: Token::Minus, value: -1, index }
}

pub fn op_dump(index: usize) -> Operation {
    Operation { token: Token::Dump, value: -1, index }
}

pub fn op_push(index: usize, value: i64) -> Operation {
    Operation { token: Token::Push, value, index }
}