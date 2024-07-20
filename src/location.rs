#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub file: String,
    pub row: usize,
    pub col: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}:", self.file, self.row, self.col)
    }
}