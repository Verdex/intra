
use proc_macro::*;

use crate::data::*;

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


pub fn parse_ident<'a>( input : &'a [TokenTree] ) -> Result<(IntraIdent<'a>, &'a [TokenTree]), Span> {

    Ok(( IntraIdent(vec![]), &input[1..]))
}
