use crate::source;

#[derive(Debug)]
pub struct Root<'src> {
    pub preamble: Preamble<'src>,
}

#[derive(Debug)]
pub struct Preamble<'src> {
    pub class: DocumentClass<'src>,
}

#[derive(Debug)]
pub struct DocumentClass<'src> {
    pub name: Ident<'src>,
    pub opts: Vec<Opt<'src>>,
}

#[derive(Debug)]
pub struct Opt<'src> {
    pub key: Ident<'src>,
    pub val: Option<Ident<'src>>,
}

#[derive(Debug)]
pub struct Ident<'src> {
    // TODO privatize
    pub src: &'src str,
    pub loc: source::Location,
}
