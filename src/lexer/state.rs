use super::error::ErrorData;
use crate::ast;

#[derive(Copy, Clone)]
pub enum State {
    Start,
    Newline,
    Whitespace,
    Par,
    Plain,
    Comment,
    MacroStart,
    MacroNormal,
    MacroSpecial,
    Literal(ast::TokenData),
    Eof,
    Fail,
    Exit,
}

impl State {
    pub fn transition(self, c: Option<char>) -> (Self, Result<Option<ast::TokenData>, ErrorData>) {
        match self {
            State::Start => self.default_transition(c, None),
            State::Par => match c {
                Some(c) if c.is_ascii_whitespace() => (State::Par, Ok(None)),
                _ => self.default_transition(c, Some(ast::TokenData::Par)),
            },
            State::Newline => match c {
                Some('\n') => (State::Par, Ok(None)),
                Some(c) if c.is_ascii_whitespace() => (State::Whitespace, Ok(None)),
                _ => self.default_transition(c, Some(ast::TokenData::Whitespace)),
            },
            State::Whitespace => match c {
                Some(c) if c.is_ascii_whitespace() => (State::Whitespace, Ok(None)),
                _ => self.default_transition(c, Some(ast::TokenData::Whitespace)),
            },
            State::Plain => self.default_transition(c, Some(ast::TokenData::Plain)),
            State::Comment => match c {
                None => (State::Eof, Ok(Some(ast::TokenData::Comment))),
                Some('\n') => (State::Start, Ok(Some(ast::TokenData::Comment))),
                Some(_) => (State::Comment, Ok(None)),
            },
            State::MacroStart => match c {
                None => (State::Fail, Err(ErrorData::UnexpectedEof)),
                Some(c) => match c {
                    '!' | ',' | ':' | ';' | '%' | '&' | '{' | '}' | '(' | ')' | '\\' | '['
                    | ']' => (State::MacroSpecial, Ok(None)),
                    _ if c.is_ascii_alphabetic() => (State::MacroNormal, Ok(None)),
                    _ => (State::Fail, Err(ErrorData::UnrecognizedMacroChar)),
                },
            },
            State::MacroNormal => match c {
                None => (State::Eof, Ok(Some(ast::TokenData::MacroNormal))),
                Some(c) if c.is_ascii_alphabetic() => (State::MacroNormal, Ok(None)),
                Some(c) if c.is_ascii_whitespace() => (State::Start, Ok(None)),
                Some(_) => self.default_transition(c, Some(ast::TokenData::MacroNormal)),
            },
            State::Literal(token) => self.default_transition(c, Some(token)),
            State::MacroSpecial => self.default_transition(c, Some(ast::TokenData::MacroSpecial)),
            State::Eof => (State::Exit, Ok(Some(ast::TokenData::Eof))),
            State::Fail => (State::Fail, Err(ErrorData::Failed)),
            State::Exit => (State::Exit, Err(ErrorData::Exited)),
        }
    }

    fn default_transition(
        &self,
        c: Option<char>,
        ok: Option<ast::TokenData>,
    ) -> (Self, Result<Option<ast::TokenData>, ErrorData>) {
        let ok = |state: State| (state, Ok(ok));
        match c {
            None => ok(State::Eof),
            Some(c) => match c {
                '\\' => ok(State::MacroStart),
                '%' => ok(State::Comment),
                '{' => ok(State::Literal(ast::TokenData::BraceL)),
                '}' => ok(State::Literal(ast::TokenData::BraceR)),
                '[' | ']' | ',' | '.' | '/' | '(' | ')' | ':' | ';' | '&' | '=' | '-' | '+'
                | '_' | '*' | '`' | '\'' | '^' | '"' | '<' | '>' | '!' | '?' | '~' | '@' => {
                    ok(State::Plain)
                }
                '\n' => ok(State::Newline),
                _ if c.is_ascii_whitespace() => ok(State::Whitespace),
                _ if c.is_ascii_alphanumeric() => ok(State::Plain),
                _ => {
                    println!("{:?}", c);
                    (State::Fail, Err(ErrorData::UnrecognizedChar))
                }
            },
        }
    }
}
