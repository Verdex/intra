
use proc_macro::*;

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

pub struct Error(Span, Vec<String>);

impl Error {
    pub fn new(span : Span, s : String) -> Self {
        Error(span, vec![s])
    }
    pub fn agument(mut self, s : String) -> Self {
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

pub struct IntraIdent<'a>(pub Vec<&'a TokenTree>);