use super::{error::Error, state::State};
use crate::{source, syntax::token};

pub struct Lexer<'src> {
    src: &'src source::Source<'src>,
    chars: std::str::Chars<'src>,
    loc: source::Location,
    start: source::Location,
    state: State,
}

impl<'a, 'src> Lexer<'src> {
    pub fn new(src: &'src source::Source<'src>) -> Lexer<'src> {
        Lexer {
            src,
            chars: src.chars(),
            state: State::Start,
            start: source::Location::start(),
            loc: source::Location::start(),
        }
    }

    pub fn source(&self) -> &'src source::Source<'src> {
        self.src
    }

    fn step(&'a mut self) -> Result<Option<token::Token<'src>>, Error> {
        let c = self.chars.next();
        let (state, result) = self.state.transition(c);
        self.state = state;

        let result = match result {
            Err(data) => Err(Error::new(data, self.loc)),
            Ok(data) => Ok(data.map(|data| {
                let src = self.src.slice(&self.start, &self.loc);
                let token = token::Token::new(data, src, self.start);
                self.start = self.loc;
                token
            })),
        };

        if let Some(c) = c {
            self.loc.step(c);
        }

        result
    }

    pub fn next(&'a mut self) -> Result<token::Token<'src>, Error> {
        loop {
            if let Some(token) = self.step()? {
                return Ok(token);
            }
        }
    }
}
