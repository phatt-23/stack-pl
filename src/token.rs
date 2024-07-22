use crate::location::Location;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenKind {
    Word(String),
    Integer(i32),
    String(String),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub loc: Location,
    pub kind: TokenKind,
}

impl Token {
    pub fn new_integer(value: i32, file: &String, row: usize, col: usize) -> Self {
        Self { 
            loc: Location::new(file, row, col),
            kind: TokenKind::Integer(value), 
        }
    }
    pub fn new_word(value: &String, file: &String, row: usize, col: usize) -> Self {
        Self { 
            loc: Location::new(file, row, col),
            kind: TokenKind::Word(value.to_string()), 
        }
    }
    pub fn new_string(value: &String, file: &String, row: usize, col: usize) -> Self {
        let value = value.replace("\\n", "\n")
                         .replace("\\t", "\t")
                         .replace("\\r", "\r");
        Self { 
            loc: Location::new(file, row, col),
            kind: TokenKind::String(value), 
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = format!("{:?}", self.kind);
        write!(f, "Token {}\tkind: {:<10}", 
            self.loc, kind
        )
    }
}
