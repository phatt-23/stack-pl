use super::Token;

#[derive(Debug)]
pub struct Operation {
    pub index: usize,
    pub token: Token,
    pub value: i64,
}

pub fn op_else(index: usize) -> Operation {
    Operation { 
        token: Token::Else, 
        value: 0, 
        index,
    }    
}

pub fn op_if(index: usize) -> Operation {
    Operation { 
        token: Token::If, 
        value: 0, 
        index,
    }    
}

pub fn op_end(index: usize) -> Operation {
    Operation { 
        token: Token::End, 
        value: 0, 
        index,
    }    
}

pub fn op_eq(index: usize) -> Operation {
    Operation { 
        token: Token::Eq, 
        value: 0, 
        index,
    }
}

pub fn op_le(index: usize) -> Operation {
    Operation { 
        token: Token::Le, 
        value: 0, 
        index,
    }
}

pub fn op_gr(index: usize) -> Operation {
    Operation { 
        token: Token::Gr, 
        value: 0,
        index,
    }
}

pub fn op_plus(index: usize) -> Operation {
    Operation { 
        token: Token::Plus, 
        value: 0,
        index,
    }
}

pub fn op_minus(index: usize) -> Operation {
    Operation { 
        token: Token::Minus, 
        value: 0,
        index,
    }
}

pub fn op_dump(index: usize) -> Operation {
    Operation { 
        token: Token::Dump, 
        value: 0,
        index,
    }
}

pub fn op_push(index: usize, value: i64) -> Operation {
    Operation { 
        token: Token::Push, 
        value,
        index,
    }
}