
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

macro_rules! alt {
    ($input:ident => $($p:ident)|+) => {
        #[allow(unused_assignments)]
        loop {
            let mut error : Option<Error> = None;
            let mut idents = vec![];
            $(
                match $p($input.clone()) {
                    Ok((v, i)) => {
                        break Ok((v, i));
                    },
                    Err(e) => { 
                        idents.push(stringify!($p));
                        error = Some(e); 
                    },
                };
            )+
            let error = Error::new(error.unwrap().span(), format!("alt failed all parses: {}", idents.join(", ")));
            break Err(error);
        }
    };
}

fn parse_gt<'a>( input : Input<'a> ) -> ParseResult<'a, &'a TokenTree> {
    match input.input() {
        [x @ TokenTree::Punct(p), rest @ ..] if p.as_char() == '>' => Ok((x, Input::new(rest, p.span()))),
        [x, ..] => Err(Error::new(x.span(), "expected '>'".to_owned())),
        [] => input.end_of_stream(), 
    }
}

fn parse_eq<'a>( input : Input<'a> ) -> ParseResult<'a, &'a TokenTree> {
    match input.input() {
        [x @ TokenTree::Punct(p), rest @ ..] if p.as_char() == '=' => Ok((x, Input::new(rest, p.span()))),
        [x, ..] => Err(Error::new(x.span(), "expected '='".to_owned())),
        [] => input.end_of_stream(), 
    }
}

fn parse_arrow<'a>( input : Input<'a> ) -> ParseResult<'a, Vec<&'a TokenTree>> {
    seq!(input => eq <= parse_eq, gt <= parse_gt => {
        vec![ eq, gt ]
    })
}

fn parse_app<'a>( input : Input<'a> ) -> ParseResult<'a, &'a TokenTree> {
    match input.input() {
        [x @ TokenTree::Punct(p), rest @ ..] if p.as_char() == '$' => Ok((x, Input::new(rest, p.span()))),
        [x, ..] => Err(Error::new(x.span(), "expected '$'".to_owned())),
        [] => input.end_of_stream(), 
    }
}

fn parse_semicolon<'a>( input : Input<'a> ) -> ParseResult<'a, &'a TokenTree> {
    match input.input() { 
        [t @ TokenTree::Punct(p), rest @ ..] if p.as_char() == ';' => Ok((t, Input::new(rest, p.span()))),
        [x, ..] => Err(Error::new(x.span(), "expected ';'".to_owned())),
        [] => input.end_of_stream(), 
    }
}

fn parse_comma<'a>( input : Input<'a> ) -> ParseResult<'a, &'a TokenTree> {
    match input.input() { 
        [t @ TokenTree::Punct(p), rest @ ..] if p.as_char() == ',' => Ok((t, Input::new(rest, p.span()))),
        [x, ..] => Err(Error::new(x.span(), "expected ','".to_owned())),
        [] => input.end_of_stream(), 
    }
}

fn parse_colon<'a>( input : Input<'a> ) -> ParseResult<'a, &'a TokenTree> {
    match input.input() { 
        [t @ TokenTree::Punct(p), rest @ ..] if p.as_char() == ':' => Ok((t, Input::new(rest, p.span()))),
        [x, ..] => Err(Error::new(x.span(), "expected ':'".to_owned())),
        [] => input.end_of_stream(), 
    }
}

fn parse_colon_colon<'a>( input : Input<'a> ) -> ParseResult<'a, Vec<&'a TokenTree>> {
    let (colon, input) = parse_colon(input).map_err(|err| err.augment("'::' is missing first ':'".to_owned()))?;
    parse_colon(input).map_err(|err| err.augment("'::' is missing second ':'".to_owned()))
                      .map(|(colon_2, input)| (vec![colon, colon_2], input))
}

fn parse_sym<'a>( input : Input<'a> ) -> ParseResult<'a, &'a TokenTree> {
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

fn map<'a, T, S, F : Fn(T) -> S>( parser : Parser!('a, T), f : F ) -> Parser!('a, S) {
    move |input : Input<'a>| {
        match parser(input) {
            Ok((v, i)) => Ok((f(v), i)),
            Err(e) => Err(e),
        }
    }
}

fn parse_pre_map<'a>( input : Input<'a> ) -> ParseResult<'a, IntraIdent> {
    seq!( input => ident <= parse_ident, _app <= parse_app => {
        ident
    })
}

fn parse_pattern_bracket<'a>( input : Input<'a> ) -> ParseResult<'a, Pattern> {
    match input.input() {
        [TokenTree::Group(g), rest @ ..] if g.delimiter() == Delimiter::Bracket
            => Ok((Pattern::new(g.stream().to_string()), Input::new(rest, g.span()))),
        [x, ..] => Err(Error::new(x.span(), "expected '{ <Group> }'".to_owned())),
        [] => input.end_of_stream(),
    }
}

fn parse_pattern_element<'a>( input : Input<'a> ) -> ParseResult<'a, (Option<IntraIdent>, Pattern, Vec<IntraIdent>)> {
    let maybe_ident = maybe(parse_pre_map);

    seq!( input => m_ident <= maybe_ident, pattern <= parse_pattern_bracket, idents <= parse_ident_list => {
        (m_ident, pattern, idents)
    })
}

fn parse_execute<'a>( input : Input<'a> ) -> ParseResult<'a, Execute> {
    match input.input() {
        [TokenTree::Group(g), rest @ ..] if g.delimiter() == Delimiter::Brace 
            => Ok((Execute::new(g.stream().to_string()), Input::new(rest, g.span()))),
        [x, ..] => Err(Error::new(x.span(), "expected '{ <Group> }'".to_owned())),
        [] => input.end_of_stream(),
    }
}

fn parse_ident_list<'a>( input : Input<'a> ) -> ParseResult<'a, Vec<IntraIdent>> {
    fn parse_ident_comma<'a>( input : Input<'a> ) -> ParseResult<'a, IntraIdent> {
        seq!( input => ident <= parse_ident, _comma <= parse_comma => {
            ident
        })
    }

    let ident_commas = zero_or_more(parse_ident_comma);
    let maybe_ident = maybe(parse_ident);

    seq!( input => idents <= ident_commas, m_ident <= maybe_ident => {
        let mut idents = idents;
        match m_ident {
            Some(ident) => { idents.push(ident); idents },
            None => idents,
        }
    })
}

fn parse_ident<'a>( input : Input<'a> ) -> ParseResult<'a, IntraIdent> {
    let mcc = maybe(parse_colon_colon);
    let tail = zero_or_more(parse_colon_colon_sym);

    seq!(input => maybe_cc <= mcc, sym <= parse_sym, tails <= tail => {
        let mut tails = tails.into_iter().flatten().collect::<Vec<_>>();
        tails.insert(0, sym);
        match maybe_cc {
            Some(mut cc) => { 
                cc.append(&mut tails);
                IntraIdent::new(cc.into_iter().map(|x| x.to_string()).collect())
            },
            None => { IntraIdent::new(tails.into_iter().map(|x| x.to_string()).collect()) },
        }
    })
}

fn parse_execute_or_pattern_list<'a>( input : Input<'a> ) -> ParseResult<'a, Vec<AtomElement>> {
    fn parse_execute_or_pattern<'a>( input : Input<'a> ) -> ParseResult<'a, AtomElement> {
        let pattern = map(parse_pattern_element, |(pre_map, pattern, next)| AtomElement::Pattern { pre_map, pattern, next });
        let execute = map(parse_execute, |x| AtomElement::Execute(x));
        alt!( input => pattern | execute )
    } 
    fn parse_execute_or_pattern_semicolon<'a>( input : Input<'a> ) -> ParseResult<'a, AtomElement> {
        seq!( input => execute_or_pattern <= parse_execute_or_pattern, _semicolon <= parse_semicolon => {
            execute_or_pattern
        })
    }
    let list = zero_or_more(parse_execute_or_pattern_semicolon);
    seq!( input => execute_or_pattern_list <= list, tail_execute_or_pattern <= parse_execute_or_pattern => {
        let mut xs = execute_or_pattern_list;
        xs.push(tail_execute_or_pattern);
        xs
    })
}

pub fn parse_atom<'a>( input : Input<'a> ) -> ParseResult<'a, Atom> {
    seq!( input => init <= parse_ident
                 , _arrow_1 <= parse_arrow
                 , seq <= parse_execute_or_pattern_list 
                 , _arrow_2 <= parse_arrow
                 , resolve <= parse_execute
                 => 
                 { Atom { init, seq, resolve } })
}