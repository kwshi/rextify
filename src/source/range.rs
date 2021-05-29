use super::location;
#[derive(Debug, Copy, Clone)]
pub struct Range {
    start: location::Location,
    end: location::Location,
}

impl Range {
    pub fn new(start: location::Location, end: location::Location) -> Range {
        Range { start, end }
    }
}
