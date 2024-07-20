#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub file: String,
    pub row: usize,
    pub col: usize,
}

impl Location {
    pub fn new(file: &String, row: usize, col: usize) -> Self {
        Self {
            file: file.clone(),
            row: row + 1, 
            col: col + 1
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}:", self.file, self.row, self.col)
    }
}