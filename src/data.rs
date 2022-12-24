
use proc_macro::*;

#[derive(Clone)]
pub struct Input<'a>(&'a [TokenTree], Span);

impl<'a> Input<'a> {
    pub fn new(input : &'a [TokenTree], span : Span) -> Self {
        Input(input, span)
    }
    pub fn input(&self) -> &'a [TokenTree] {
        self.0
    }
    pub fn end_of_stream<T>(self) -> Result<T, Error> {
        Err(Error::new(self.1, "unexpected end of stream".to_owned()))
    }
}

#[derive(Debug)]
pub struct Error(Span, Vec<String>);

impl Error {
    pub fn new(span : Span, s : String) -> Self {
        Error(span, vec![s])
    }
    pub fn augment(mut self, s : String) -> Self {
        self.1.push(s);
        self
    }
    pub fn message(&self) -> String {
        self.1.join("\n")
    }
    pub fn span(self) -> Span {
        self.0
    } 
}

#[derive(Clone)]
pub struct IntraIdent(Vec<TokenTree>);

impl IntraIdent {
    pub fn new(input : Vec<TokenTree>) -> Self {
        IntraIdent(input)
    }
    pub fn trees(self) -> Vec<TokenTree> {
        self.0
    }
}

pub struct Execute(Vec<TokenTree>);

impl Execute {
    pub fn new(input : Vec<TokenTree>) -> Self { Execute(input) }
    pub fn trees(self) -> Vec<TokenTree> { self.0 }
}

pub struct Pattern(Vec<TokenTree>);

impl Pattern {
    pub fn new(input : Vec<TokenTree>) -> Self { Pattern(input) }
    pub fn trees(self) -> Vec<TokenTree> { self.0 }
}

pub enum AtomElement {
    Execute(Execute),
    Pattern { pre_map : Option<IntraIdent>, pattern : Pattern, next : Vec<IntraIdent> },
}

pub struct Atom {
    pub init : IntraIdent,
    pub seq : Vec<AtomElement>,
    pub resolve : Execute,
}