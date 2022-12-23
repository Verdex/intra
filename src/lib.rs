
mod data;
mod parsing;

use proc_macro::*;

use crate::data::*;
use crate::parsing::*;

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

    let first = &input[0];
    let second = &input[1];
    


    let z = parse_colon_colon(Input::new(&input[2..], second.span()));

    match z {
        Err(s) => { 
            gen_compile_error(s)
        },
        _ => { 
            "".parse().unwrap()
        },
    }
}

