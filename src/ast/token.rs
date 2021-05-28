#[derive(Debug)]
pub struct Token<'src> {
    src: &'src str,
    data: TokenData,
}

#[derive(Debug, Copy, Clone)]
pub enum TokenData {
    Eof,
    BraceL,
    BraceR,
    Plain,
    MacroNormal,
    MacroSpecial,
    Whitespace,
    Par,
    Comment,
}

impl<'src> Token<'src> {
    pub fn new(src: &'src str, data: TokenData) -> Token<'src> {
        Token { src, data }
    }

    pub fn data(&self) -> TokenData {
        self.data
    }
}
