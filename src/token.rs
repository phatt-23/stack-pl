use super::location::Location;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Word,
    Int,
    // Str,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Int(i32),
    Str(String),
    // Str(String),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub loc: Location,
    pub tok_type: TokenType,
    pub value: TokenValue,
}

impl Token {
    pub fn new_int(tok_type: TokenType, value: i32, file: &String, row: usize, col: usize) -> Self {
        Self { 
            tok_type, 
            value: TokenValue::Int(value), 
            loc: Location::new(file, row, col)
        }
    }
    pub fn new_word(tok_type: TokenType, value: &String, file: &String, row: usize, col: usize) -> Self {
        Self { 
            tok_type, 
            value: TokenValue::Str(value.to_string()), 
            loc: Location::new(file, row, col)
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tok_type = format!("{:?}", self.tok_type);
        let value = format!("{:?}", self.value);
        write!(f, "Token \tloc: {} \ttok_type: {:<10} \tvalue: {:<10}", 
            self.loc, tok_type, value
        )
    }
}