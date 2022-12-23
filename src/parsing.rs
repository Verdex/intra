
use proc_macro::*;

use crate::data::*;

//type Result<'a, T> = Result<(&'a TokenTree, Input<'a>), Error>;

macro_rules! Parser {
    ($life:lifetime, $t:ty) => {
        impl Fn(Input<$life>) -> Result<($t, Input<$life>), Error>
    };
}

pub fn parse_colon<'a>( input : Input<'a> ) -> Result<(&'a TokenTree, Input<'a>), Error> {
    match input.input() { 
        [t @ TokenTree::Punct(p), rest @ ..] if p.as_char() == ':' => Ok((t, Input::new(rest, p.span()))),
        [x, ..] => Err(Error::new(x.span(), "expected ':'".to_owned())),
        [] => input.end_of_stream(), 
    }
}

pub fn parse_colon_colon<'a>( input : Input<'a> ) -> Result<(Vec<&'a TokenTree>, Input<'a>), Error> {
    let (colon, input) = parse_colon(input).map_err(|err| err.agument("'::' is missing first ':'".to_owned()))?;
    parse_colon(input).map_err(|err| err.agument("'::' is missing second ':'".to_owned()))
                      .map(|(colon_2, input)| (vec![colon, colon_2], input))
}

pub fn parse_sym<'a>( input : Input<'a> ) -> Result<(&'a TokenTree, Input<'a>), Error> {
    match input.input() {
        [t @ TokenTree::Ident(_), rest @ ..] => Ok((t, Input::new(rest, t.span()))),
        [x, ..] => Err(Error::new(x.span(), "expected '<ident>'".to_owned())),
        [] => input.end_of_stream(),
    }
}

fn maybe<'a, T>( parser : Parser!('a, T) ) -> Parser!('a, Option<T>) {
    move |input : Input<'a>| {
        match parser(input.clone()) {
            Ok((t, i)) => Ok((Some(t), i)),
            Err(_) => Ok((None, input)),
        }
    }
}

//fn zero_or_more<'a, T>( parser : Parser!('a, T))

pub fn parse_ident<'a>( input : Input<'a> ) -> Result<(IntraIdent<'a>, Input<'a>), Error> {
//    let mut identifier = vec![];
    let z = maybe( parse_sym );

    z(input)
}