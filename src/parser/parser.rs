use super::error::{Error, ErrorData};
use crate::{
    lexer,
    syntax::{ast, token},
};

pub struct Parser<'src> {
    lexer: lexer::Lexer<'src>,
    peek: Option<token::Token<'src>>,
}

// TODO pure, state-based parser?

impl<'a, 'src> Parser<'src> {
    pub fn new(lexer: lexer::Lexer<'src>) -> Self {
        Parser { lexer, peek: None }
    }

    fn peek(&'a mut self) -> Result<&'a token::Token<'src>, Error<'src>> {
        if let None = self.peek {
            let token = self.lexer.next()?;
            self.peek = Some(token);
        }
        Ok(self.peek.as_ref().unwrap())
    }

    fn next(&'a mut self) -> Result<token::Token<'src>, Error<'src>> {
        self.peek()?;
        Ok(self.peek.take().unwrap())
    }

    fn skip_whitespace(&'a mut self) -> Result<Option<token::Token<'src>>, Error<'src>> {
        Ok(if self.peek()?.is_whitespace() {
            Some(self.next()?)
        } else {
            None
        })
    }

    fn parse_document_class(&'a mut self) -> Result<ast::DocumentClass<'src>, Error<'src>> {
        self.skip_whitespace()?;
        Self::expect_macro(self.next()?, "documentclass")?;
        let opts = self.parse_optional_opts()?;
        self.skip_whitespace()?;

        let name = self.parse_group(|p| p.parse_ident())?;
        Ok(ast::DocumentClass { name, opts })
    }

    fn parse_opts(
        &'a mut self,
        close: impl Fn(&token::Token) -> bool,
    ) -> Result<Vec<ast::Opt<'src>>, Error<'src>> {
        let mut opts = Vec::new();
        self.skip_whitespace()?;

        loop {
            // TODO get rid of close; simply parse until no more valid options?
            if close(self.peek()?) {
                return Ok(opts);
            }

            opts.push(self.parse_opt()?);
            self.skip_whitespace()?;

            let next = self.peek()?;
            match next.data() {
                token::TokenData::Comma => {
                    self.next()?;
                    self.skip_whitespace()?;
                }
                _ if close(next) => {
                    return Ok(opts);
                }
                _ => return Err(Error::new(ErrorData::UnexpectedToken, next.loc())),
            }
        }
    }

    fn parse_group<T>(
        &'a mut self,
        f: impl Fn(&mut Self) -> Result<T, Error<'src>>,
    ) -> Result<T, Error<'src>> {
        let left = self.next()?;
        match left.data() {
            token::TokenData::BraceL => Ok(()),
            _ => Err(Error::new(ErrorData::Misc("expecting BraceL"), left.loc())),
        }?;

        let body = f(self)?;

        let right = self.next()?;
        match right.data() {
            token::TokenData::BraceR => Ok(()),
            _ => Err(Error::new(ErrorData::Misc("expecting BraceR"), right.loc())),
        }?;

        Ok(body)
    }

    fn parse_opt(&'a mut self) -> Result<ast::Opt<'src>, Error<'src>> {
        let key = self.parse_ident()?;
        self.skip_whitespace()?;
        match self.peek()?.data() {
            token::TokenData::Equal => {
                self.next()?;
                self.skip_whitespace()?;
                // TODO
                Ok(ast::Opt { key, val: None })
            }
            _ => Ok(ast::Opt { key, val: None }),
        }
    }

    fn parse_optional_opts(&'a mut self) -> Result<Vec<ast::Opt<'src>>, Error<'src>> {
        self.parse_optional(|p| {
            p.parse_opts(|close| match close.data() {
                token::TokenData::BracketR => true,
                _ => false,
            })
        })
        .map(Option::unwrap_or_default)
    }

    fn parse_optional<T>(
        &'a mut self,
        f: impl Fn(&mut Self) -> Result<T, Error<'src>>,
    ) -> Result<Option<T>, Error<'src>> {
        let left = self.peek()?;
        match left.data() {
            token::TokenData::BracketL => {
                self.next()?;
                self.skip_whitespace()?;
            }
            _ => return Ok(None),
        }

        let body = f(self)?;

        let right = self.next()?;
        match right.data() {
            token::TokenData::BracketR => Ok(()),
            _ => Err(Error::new(
                ErrorData::Misc("expecting BracketR"),
                right.loc(),
            )),
        }?;

        Ok(Some(body)) // once told me the world is gonna roll me
    }

    fn parse_preamble(&'a mut self) -> Result<ast::Preamble<'src>, Error<'src>> {
        let class = self.parse_document_class()?;
        Ok(ast::Preamble { class })
    }

    fn parse_ident(&'a mut self) -> Result<ast::Ident<'src>, Error<'src>> {
        let first = self.next()?;
        let start = if first.is_ident() {
            Ok(first.loc())
        } else {
            Err(Error::new(ErrorData::ExpectingIdent, first.loc()))
        }?;

        let end = loop {
            while self.peek()?.is_ident() {
                self.next()?;
            }
            match self.skip_whitespace()? {
                None => break self.peek()?.loc(),
                // TODO disallow par?
                Some(token) => {
                    if self.peek()?.is_ident() {
                        self.next()?;
                    } else {
                        break token.loc();
                    }
                }
            };
        };

        Ok(ast::Ident {
            src: self.lexer.source().slice(&start, &end),
            loc: start,
        })
    }

    fn expect_macro(token: token::Token<'src>, name: &'static str) -> Result<(), Error<'src>> {
        match token.macro_name() {
            Some(s) if name == s => Ok(()),
            _ => Err(Error::new(ErrorData::ExpectingMacro(name), token.loc())),
        }
    }

    pub fn parse(mut self) -> Result<ast::Root<'src>, Error<'src>> {
        let preamble = self.parse_preamble()?;
        Ok(ast::Root { preamble })
    }
}
