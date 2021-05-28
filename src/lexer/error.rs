use crate::source::Location;

#[derive(thiserror::Error, Debug)]
#[error("aa")]
pub struct Error {
    loc: Location,
    data: ErrorData,
}

#[derive(Debug)]
pub enum ErrorData {
    UnexpectedEof,
    UnrecognizedChar,
    UnrecognizedMacroChar,
    Failed,
    Exited,
}

impl Error {
    pub fn new(data: ErrorData, loc: Location) -> Error {
        Error { data, loc }
    }

    pub fn loc(&self) -> Location {
        self.loc
    }
}
