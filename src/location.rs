#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub file: String,
    pub row: usize,
    pub col: usize,
}

#[macro_export]
macro_rules! goto_loc {
    () => {
        Location {
            file: file!().to_string(), 
            row: line!() as usize, 
            col: column!() as usize
        }
    }
}

pub(crate) use goto_loc;


impl Location {
    pub fn new(file: &str, row: usize, col: usize) -> Self {
        Self {
            file: file.to_string(),
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
