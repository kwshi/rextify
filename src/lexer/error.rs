use crate::source::Location;

#[derive(thiserror::Error, Debug)]
#[error("lex error")]
pub struct Error {
    loc: Location,
    data: ErrorData,
}

#[derive(Debug)]
pub enum ErrorData {
    UnexpectedEof,
    UnrecognizedChar,
    UnrecognizedMacroChar,
    TripleDollar,
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

    pub fn data(self) -> ErrorData {
        self.data
    }
}
