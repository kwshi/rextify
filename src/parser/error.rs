use crate::{lexer, source};

#[derive(thiserror::Error, Debug)]
#[error("parse error")]
pub struct Error<'src> {
    loc: source::Location,
    data: ErrorData<'src>,
}

#[derive(thiserror::Error, Debug)]
pub enum ErrorData<'src> {
    #[error("lexing error")]
    Lex(lexer::ErrorData),

    #[error("expecting \\documentclass")]
    ExpectingMacro(&'src str),

    #[error("expecting ident token")]
    ExpectingIdent,

    #[error("unexpected token")]
    UnexpectedToken,

    // TODO improve
    #[error("misc")]
    Misc(&'static str),
}

impl<'src> Error<'src> {
    pub fn new(data: ErrorData<'src>, loc: source::Location) -> Error<'src> {
        Error { data, loc }
    }
}

impl<'src> From<lexer::Error> for Error<'src> {
    fn from(err: lexer::Error) -> Error<'src> {
        Error {
            loc: err.loc(),
            data: ErrorData::Lex(err.data()),
        }
    }
}
