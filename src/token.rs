use crate::location::Location;
use crate::keyword::KeywordType;
use crate::intrinsic::IntrinsicType;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenKind {
    Word(String),
    Integer64(i64),
    Integer32(i32),
    String(String),
    Char(char),
    KeyWord(KeywordType),
    Intrinsic(IntrinsicType)
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub loc: Location,
    pub kind: TokenKind,
    pub expanded: usize,
}

impl Token {
    #[must_use]
    pub fn new_integer64(value: i64, loc: &Location) -> Self {
        Self { 
            loc: loc.clone(),
            kind: TokenKind::Integer64(value), 
            expanded: 0,
        }
    }
    #[must_use]
    pub fn new_integer32(value: i32, loc: &Location) -> Self {
        Self { 
            loc: loc.clone(),
            kind: TokenKind::Integer32(value), 
            expanded: 0,
        }
    }
    #[must_use]
    pub fn new_word(value: &String, loc: &Location) -> Self {
        Self { 
            loc: loc.clone(),
            kind: TokenKind::Word(value.to_string()), 
            expanded: 0,
        }
    }
    #[must_use]
    pub fn new_char(value: char, loc: &Location) -> Self {
        Self { 
            loc: loc.clone(),
            kind: TokenKind::Char(value), 
            expanded: 0,
        }
    }
    #[must_use]
    pub fn new_string(value: &str, loc: &Location) -> Self {
        let value = value.replace("\\n", "\n")
                         .replace("\\t", "\t")
                         .replace("\\r", "\r")
                         .replace("\\0", "\0");
        Self { 
            loc: loc.clone(),
            kind: TokenKind::String(value), 
            expanded: 0,
        }
    }
    #[must_use]
    pub fn new_keyword(value: KeywordType, loc: &Location) -> Self {
        Self { 
            loc: loc.clone(),
            kind: TokenKind::KeyWord(value), 
            expanded: 0,
        }
    }
    #[must_use]
    pub fn new_intrinsic(value: IntrinsicType, loc: &Location) -> Self {
        Self {
            loc: loc.clone(),
            kind: TokenKind::Intrinsic(value),
            expanded: 0,
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
