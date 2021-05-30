use super::error::ErrorData;
use crate::syntax::token;

#[derive(Copy, Clone)]
pub enum State {
    Start,
    Whitespace { newlines: usize },
    Plain,
    Comment,
    MacroStart,
    MacroNormal,
    MacroSpecial,
    Literal(token::TokenData),
    DollarSingle,
    DollarDouble,
    Eof,
    Fail,
    Exit,
}

impl State {
    pub fn transition(
        self,
        c: Option<char>,
    ) -> (Self, Result<Option<token::TokenData>, ErrorData>) {
        match self {
            State::Start => self.default_transition(c, None),
            State::Whitespace { newlines } => match c {
                Some('\n') => (
                    State::Whitespace {
                        newlines: newlines + 1,
                    },
                    Ok(None),
                ),
                Some(c) if c.is_ascii_whitespace() => (State::Whitespace { newlines }, Ok(None)),
                _ => self.default_transition(
                    c,
                    Some(token::TokenData::Whitespace { par: newlines >= 2 }),
                ),
            },
            State::Plain => self.default_transition(c, Some(token::TokenData::Plain)),
            State::Comment => match c {
                None => (State::Eof, Ok(Some(token::TokenData::Comment))),
                Some('\n') => (State::Start, Ok(Some(token::TokenData::Comment))),
                Some(_) => (State::Comment, Ok(None)),
            },
            State::DollarSingle => match c {
                Some('$') => (State::DollarDouble, Ok(None)),
                _ => self.default_transition(c, Some(token::TokenData::DollarSingle)),
            },
            State::DollarDouble => match c {
                Some('$') => (State::Fail, Err(ErrorData::TripleDollar)),
                _ => self.default_transition(c, Some(token::TokenData::DollarDouble)),
            },
            State::MacroStart => match c {
                None => (State::Fail, Err(ErrorData::UnexpectedEof)),
                Some(c) => match c {
                    '!' | ',' | ':' | ';' | '%' | '&' | '{' | '}' | '(' | ')' | '\\' | '['
                    | ']' | '$' => (State::MacroSpecial, Ok(None)),
                    _ if c.is_ascii_alphabetic() => (State::MacroNormal, Ok(None)),
                    _ => (State::Fail, Err(ErrorData::UnrecognizedMacroChar)),
                },
            },
            State::MacroNormal => match c {
                None => (State::Eof, Ok(Some(token::TokenData::MacroNormal))),
                Some(c) if c.is_ascii_alphabetic() => (State::MacroNormal, Ok(None)),
                Some(c) if c.is_ascii_whitespace() => (State::Start, Ok(None)),
                Some(_) => self.default_transition(c, Some(token::TokenData::MacroNormal)),
            },
            State::Literal(token) => self.default_transition(c, Some(token)),
            State::MacroSpecial => self.default_transition(c, Some(token::TokenData::MacroSpecial)),
            State::Eof => (State::Exit, Ok(Some(token::TokenData::Eof))),
            State::Fail => (State::Fail, Err(ErrorData::Failed)),
            State::Exit => (State::Exit, Err(ErrorData::Exited)),
        }
    }

    fn default_transition(
        &self,
        c: Option<char>,
        ok: Option<token::TokenData>,
    ) -> (Self, Result<Option<token::TokenData>, ErrorData>) {
        let ok = |state: State| (state, Ok(ok));
        match c {
            None => ok(State::Eof),
            Some(c) => match c {
                '\\' => ok(State::MacroStart),
                '$' => ok(State::DollarSingle),
                '%' => ok(State::Comment),
                '{' => ok(State::Literal(token::TokenData::BraceL)),
                '}' => ok(State::Literal(token::TokenData::BraceR)),
                ',' => ok(State::Literal(token::TokenData::Comma)),
                '[' => ok(State::Literal(token::TokenData::BracketL)),
                ']' => ok(State::Literal(token::TokenData::BracketR)),
                '=' => ok(State::Literal(token::TokenData::Equal)),
                '.' | '/' | '(' | ')' | ':' | ';' | '&' | '-' | '+' | '_' | '*' | '`' | '\''
                | '^' | '"' | '<' | '>' | '!' | '?' | '~' | '@' => ok(State::Plain),
                '\n' => ok(State::Whitespace { newlines: 1 }),
                _ if c.is_ascii_whitespace() => ok(State::Whitespace { newlines: 0 }),
                _ if c.is_ascii_alphanumeric() => ok(State::Plain),
                _ => {
                    println!("{:?}", c);
                    (State::Fail, Err(ErrorData::UnrecognizedChar))
                }
            },
        }
    }
}
