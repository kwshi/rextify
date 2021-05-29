use super::error::{Error, ErrorData};
use crate::{
    lexer,
    syntax::{ast, token},
};

pub struct Parser<'src> {
    lexer: lexer::Lexer<'src>,
    peek: Option<token::Token<'src>>,
}

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
        Ok(if self.peek()?.is_whitespace_like() {
            Some(self.next()?)
        } else {
            None
        })
    }

    //fn peek(&'a self) -> Result<()> {}

    fn parse_document_class(&'a mut self) -> Result<ast::DocumentClass<'src>, Error<'src>> {
        self.skip_whitespace()?;
        Self::expect_macro(self.next()?, "documentclass")?;
        let opts = self.parse_optional_opts()?;
        self.skip_whitespace()?;

        let brace = self.next()?;
        match brace.data() {
            token::TokenData::BraceL => Ok(()),
            _ => Err(Error::new(ErrorData::Misc("expecting BraceL"), brace.loc())),
        }?;

        let name = self.parse_ident()?;

        let brace = self.next()?;
        match brace.data() {
            token::TokenData::BraceR => Ok(()),
            _ => Err(Error::new(ErrorData::Misc("expecting BraceR"), brace.loc())),
        }?;

        Ok(ast::DocumentClass { name, opts })
    }

    fn parse_opts(
        &'a mut self,
        close: impl Fn(&token::Token) -> bool,
    ) -> Result<Vec<ast::Opt<'src>>, Error<'src>> {
        let mut opts = Vec::new();
        self.skip_whitespace()?;

        loop {
            if close(self.peek()?) {
                self.next()?;
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
                    self.next()?;
                    return Ok(opts);
                }
                _ => return Err(Error::new(ErrorData::UnexpectedToken, next.loc())),
            }
        }
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
        let token = self.peek()?;
        match token.data() {
            token::TokenData::BracketL => {
                self.next()?;
                self.skip_whitespace()?;
                self.parse_opts(|close| match close.data() {
                    token::TokenData::BracketR => true,
                    _ => false,
                })
            }
            _ => Ok(Vec::new()),
        }
    }

    //fn parse_group(&'a mut self)

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
