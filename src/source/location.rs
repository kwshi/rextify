#[derive(Debug, Copy, Clone)]
pub struct Location {
    pos: usize,
    row: usize,
    col: usize,
}

impl Location {
    pub fn start() -> Location {
        Location {
            pos: 0,
            row: 0,
            col: 0,
        }
    }

    pub fn step(&mut self, c: char) {
        self.pos += c.len_utf8();
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

    pub fn pos(&self) -> usize {
        self.pos
    }
}
