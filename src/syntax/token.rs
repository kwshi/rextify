use crate::source;

#[derive(Debug)]
pub struct Token<'src> {
    src: &'src str,
    loc: source::Location,
    data: TokenData,
}

#[derive(Debug, Copy, Clone)]
pub enum TokenData {
    Eof,
    BraceL,
    BraceR,
    BracketL,
    BracketR,
    Equal,
    Comma,
    Plain,
    MacroNormal,
    MacroSpecial,
    Whitespace,
    Par,
    DollarSingle,
    DollarDouble,
    Comment,
}

impl<'src> Token<'src> {
    pub fn new(data: TokenData, src: &'src str, loc: source::Location) -> Token<'src> {
        Token { src, data, loc }
    }

    pub fn data(&self) -> TokenData {
        self.data
    }

    pub fn loc(&self) -> source::Location {
        self.loc
    }

    pub fn is_whitespace_like(&self) -> bool {
        match self.data {
            TokenData::Whitespace | TokenData::Par => true,
            _ => false,
        }
    }

    pub fn macro_name(&self) -> Option<&'src str> {
        match self.data {
            TokenData::MacroNormal | TokenData::MacroSpecial => Some(&self.src[1..]),
            _ => None,
        }
    }

    pub fn plain(&self) -> Option<&'src str> {
        match self.data {
            TokenData::Plain => Some(self.src),
            _ => None,
        }
    }

    pub fn is_ident(&self) -> bool {
        match self.data {
            TokenData::Plain => self.src.chars().all(|c| match c {
                '-' | '_' | '\'' | '.' | '/' => true,
                _ if c.is_ascii_alphanumeric() => true,
                _ => false,
            }),
            _ => false,
        }
    }
}
