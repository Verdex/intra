
mod data;
mod parsing;
mod gen;

use proc_macro::*;

use crate::data::*;
use crate::parsing::*;
use crate::gen::*;

fn gen_compile_error(error : Error) -> TokenStream {
    let mut code = format!("compile_error!(\"{}\");", error.message())
        .parse::<TokenStream>()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();

    let span = error.span();
    for c in &mut code {
        c.set_span(span);
    }
    code.into_iter().collect()
}

#[proc_macro]
pub fn atom( input : TokenStream ) -> TokenStream {
    let input = input.into_iter().collect::<Vec<_>>();

    // TODO is there an empty span or pre-span or something
    let atom = parse_atom(Input::new(&input, input[0].span()));

    match atom {
        Ok((v, _)) => { // TODO make sure entire input is consumed
            gen_atom(v)
        },
        Err(e) => {
            gen_compile_error(e)
        }
    }
}

