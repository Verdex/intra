
use proc_macro::*;

use crate::data::*;

type ParseResult<'a, T> = Result<(T, Input<'a>), Error>;

macro_rules! Parser {
    ($life:lifetime, $t:ty) => {
        impl Fn(Input<$life>) -> ParseResult<$life, $t> 
    };
}

macro_rules! seq {
    ($input:ident => $($x:ident <= $p:expr),+ => $b:block) => {
        loop {
            let mut input = $input;
            $(
                let $x = match $p(input.clone()) {
                    Ok((v, i)) => {
                        input = i;
                        v
                    },
                    Err(e) => { break Err(e.augment(format!("failure while trying to parse {}", stringify!($x)))); }
                };
            )+
            break Ok(($b, input));
        }
    };
}

pub fn parse_colon<'a>( input : Input<'a> ) -> ParseResult<'a, &'a TokenTree> {
    match input.input() { 
        [t @ TokenTree::Punct(p), rest @ ..] if p.as_char() == ':' => Ok((t, Input::new(rest, p.span()))),
        [x, ..] => Err(Error::new(x.span(), "expected ':'".to_owned())),
        [] => input.end_of_stream(), 
    }
}

pub fn parse_colon_colon<'a>( input : Input<'a> ) -> ParseResult<'a, Vec<&'a TokenTree>> {
    let (colon, input) = parse_colon(input).map_err(|err| err.augment("'::' is missing first ':'".to_owned()))?;
    parse_colon(input).map_err(|err| err.augment("'::' is missing second ':'".to_owned()))
                      .map(|(colon_2, input)| (vec![colon, colon_2], input))
}

pub fn parse_sym<'a>( input : Input<'a> ) -> ParseResult<'a, &'a TokenTree> {
    match input.input() {
        [t @ TokenTree::Ident(_), rest @ ..] => Ok((t, Input::new(rest, t.span()))),
        [x, ..] => Err(Error::new(x.span(), "expected '<ident>'".to_owned())),
        [] => input.end_of_stream(),
    }
}

fn parse_colon_colon_sym<'a>( input : Input<'a> ) -> ParseResult<'a, Vec<&'a TokenTree>> {
    seq!(input => cc <= parse_colon_colon, sym <= parse_sym => { 
        let mut cc = cc;
        cc.push(sym);
        cc
    })
}

fn maybe<'a, T>( parser : Parser!('a, T) ) -> Parser!('a, Option<T>) {
    move |input : Input<'a>| {
        match parser(input.clone()) {
            Ok((t, i)) => Ok((Some(t), i)),
            Err(_) => Ok((None, input)),
        }
    }
}

fn zero_or_more<'a, T>( parser : Parser!('a, T) ) -> Parser!('a, Vec<T>) {
    move |mut input : Input<'a>| {
        let mut ret = vec![];
        loop {
            match parser(input.clone()) {
                Ok((v, i)) => { input = i; ret.push(v); },
                Err(_) => { break; },
            }
        }
        Ok((ret, input))
    }
}

pub fn parse_ident<'a>( input : Input<'a> ) -> ParseResult<'a, IntraIdent<'a>> {
    let mcc = maybe(parse_colon_colon);
    let tail = zero_or_more(parse_colon_colon_sym);

    seq!(input => maybe_cc <= mcc, sym <= parse_sym, tails <= tail => {
        let mut tails = tails.into_iter().flatten().collect::<Vec<_>>();
        tails.insert(0, sym);
        match maybe_cc {
            Some(mut cc) => { 
                cc.append(&mut tails);
                IntraIdent::new(cc)
            },
            None => { IntraIdent::new(tails) },
        }
    })
}