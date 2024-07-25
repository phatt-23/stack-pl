use crate::location::Location;
use crate::keyword::KeywordType;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenKind {
    Word(String),
    Integer(i32),
    String(String),
    Char(char),
    KeyWord(KeywordType),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub loc: Location,
    pub kind: TokenKind,
}

impl Token {
    #[must_use]
    pub fn new_integer(value: i32, loc: &Location) -> Self {
        Self { 
            loc: loc.clone(),
            kind: TokenKind::Integer(value), 
        }
    }
    #[must_use]
    pub fn new_word(value: &String, loc: &Location) -> Self {
        Self { 
            loc: loc.clone(),
            kind: TokenKind::Word(value.to_string()), 
        }
    }
    #[must_use]
    pub fn new_char(value: char, loc: &Location) -> Self {
        Self { 
            loc: loc.clone(),
            kind: TokenKind::Char(value), 
        }
    }
    #[must_use]
    pub fn new_string(value: &str, loc: &Location) -> Self {
        let value = value.replace("\\n", "\n")
                         .replace("\\t", "\t")
                         .replace("\\r", "\r");
        Self { 
            loc: loc.clone(),
            kind: TokenKind::String(value), 
        }
    }
    #[must_use]
    pub fn new_keyword(value: KeywordType, loc: &Location) -> Self {
        Self { 
            loc: loc.clone(),
            kind: TokenKind::KeyWord(value), 
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
