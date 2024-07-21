use crate::location::Location;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Word,
    Int,
    Str,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Int(i32),
    Str(String),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub loc: Location,
    pub tok_type: TokenType,
    pub value: TokenValue,
}

impl Token {
    pub fn new_int(value: i32, file: &String, row: usize, col: usize) -> Self {
        Self { 
            tok_type: TokenType::Int, 
            value: TokenValue::Int(value), 
            loc: Location::new(file, row, col)
        }
    }
    pub fn new_word(value: &String, file: &String, row: usize, col: usize) -> Self {
        Self { 
            tok_type: TokenType::Word, 
            value: TokenValue::Str(value.clone()), 
            loc: Location::new(file, row, col)
        }
    }
    pub fn new_str(value: &String, file: &String, row: usize, col: usize) -> Self {
        let value = value.replace("\\n", "\n").replace("\\t", "\t").replace("\\r", "\r");
        Self { 
            tok_type: TokenType::Str, 
            value: TokenValue::Str(value), 
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
