
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

    let atom = parse_atom(Input::new(&input, Span::call_site()));

    match atom {
        Ok((v, input)) => { 
            if let [] = input.input() {
                gen_atom(v)
            }
            else {
                gen_compile_error(input.left_over())
            }
        },
        Err(e) => {
            gen_compile_error(e)
        }
    }
}

#[proc_macro]
pub fn seq( input : TokenStream ) -> TokenStream {
    let input = input.into_iter().collect::<Vec<_>>();

    let seq = parse_seq(Input::new(&input, Span::call_site()));

    match seq {
        Ok((v, input)) => {
            "".parse().unwrap()
        },
        Err(e) => {
            gen_compile_error(e)
        }
    }
}