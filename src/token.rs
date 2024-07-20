
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Word,
    Int,
    // Str,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Int(i64),
    Str(String),
    // Str(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub tok_type: TokenType,
    pub value: TokenValue,

    pub loc: super::location::Location,
}

impl Token {
    pub fn new_int(tok_type: TokenType, value: i64, file: &String, row: usize, col: usize) -> Self {
        Self { 
            tok_type, 
            value: TokenValue::Int(value), 
            loc: super::location::Location { file: file.to_string(), row, col }
        }
    }
    pub fn new_word(tok_type: TokenType, value: &String, file: &String, row: usize, col: usize) -> Self {
        Self { 
            tok_type, 
            value: TokenValue::Str(value.to_string()), 
            loc: super::location::Location { file: file.to_string(), row, col }
        }
    }
}