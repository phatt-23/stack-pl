// Keyword

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum KeywordType {
    If,
    Else,
    End,
    While,
    Do,
    Macro,
    Include,
}

impl KeywordType {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "if"      => Some(Self::If),
            "end"     => Some(Self::End),
            "else"    => Some(Self::Else),
            "while"   => Some(Self::While),
            "do"      => Some(Self::Do),
            "macro"   => Some(Self::Macro),
            "include" => Some(Self::Include),
            _         => None,
        }
    }
}
