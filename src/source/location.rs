#[derive(Debug, Copy, Clone)]
pub struct Location {
    row: usize,
    col: usize,
}

impl Location {
    pub fn start() -> Location {
        Location { row: 0, col: 0 }
    }

    pub fn newline(&mut self) {
        self.row += 1;
        self.col = 0;
    }

    pub fn step(&mut self, c: char) {
        match c {
            '\n' => {
                self.row += 1;
                self.col = 0;
            }
            _ => self.col += 1,
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }
}
