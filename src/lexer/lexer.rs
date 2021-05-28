use super::{error::Error, state::State};
use crate::{ast, source};

pub struct Lexer<'src> {
    src: &'src source::Source<'src>,
    chars: std::str::CharIndices<'src>,
    loc: source::Location,
    start: usize,
    state: State,
}

impl<'a, 'src> Lexer<'src> {
    pub fn new(src: &'src source::Source<'src>) -> Lexer<'src> {
        Lexer {
            src,
            chars: src.char_indices(),
            start: 0,
            state: State::Start,
            loc: source::Location::start(),
        }
    }

    fn step(&'a mut self) -> Result<Option<ast::Token<'src>>, Error> {
        let (pos, c) = match self.chars.next() {
            None => (self.src.len(), None),
            Some((i, c)) => (i, Some(c)),
        };

        let (state, result) = self.state.transition(c);
        self.state = state;

        let result = match result {
            Err(data) => Err(Error::new(data, self.loc)),
            Ok(data) => Ok(data.map(|data| {
                let start = self.start;
                self.start = pos;
                let src = self.src.slice(start, pos);
                ast::Token::new(src, data)
            })),
        };

        if let Some(c) = c {
            self.loc.step(c);
        }

        result
    }

    pub fn next(&'a mut self) -> Result<ast::Token<'src>, Error> {
        loop {
            if let Some(token) = self.step()? {
                return Ok(token);
            }
        }
    }
}
