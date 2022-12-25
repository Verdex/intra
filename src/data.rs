
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
    pub fn left_over(self) -> Error {
        Error::new(self.0.iter().last().unwrap().span(), "entire input was not consumed".to_owned())
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
pub struct IntraIdent(String);

impl IntraIdent {
    pub fn new(input : String) -> Self {
        IntraIdent(input)
    }
    pub fn code(self) -> String {
        self.0
    }
}

pub struct Execute(String);

impl Execute {
    pub fn new(input : String) -> Self { Execute(input) }
    pub fn code(self) -> String { self.0 }
}

#[derive(Clone)]
pub struct Pattern(String);

impl Pattern {
    pub fn new(input : String) -> Self { Pattern(input) }
    pub fn code(self) -> String { self.0 }
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