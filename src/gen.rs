
use proc_macro::*;

use crate::data::*;

fn use_pre_map(map : Option<IntraIdent>, ident : String) -> String {
    match map {
        Some(map) => format!( "{}({})", map.code(), ident ),
        None => ident,
    }
}

pub fn gen_atom( input : Atom ) -> TokenStream {
    let init = input.init.code();
    let mut elements = input.seq;
    let resolve = input.resolve.code();

    let mut previous = std::iter::once(vec![init])
        .chain(elements.iter()
                       .flat_map(|x| match x { AtomElement::Pattern { next, .. } => vec![next], _ => vec![] })
                       .map(|x| x.clone().into_iter().map(|y| y.code()).collect::<Vec<_>>())
              )
        .collect::<Vec<_>>();
    previous.pop();

    let mut code = resolve; 

    while elements.len() != 0 {
        match elements.pop().unwrap() {
            AtomElement::Execute(e) => { code = format!("{}\n{}", e.code(), code) },
            AtomElement::Pattern { pre_map, pattern, .. } => {
                code = previous.pop().unwrap().into_iter().map(|ident| {
                    format!("match {ident} {{
                        {pattern} => {{
                            {prev}
                        }},
                        _ => {{ }},
                    }}", ident = use_pre_map(pre_map.clone(), ident)
                       , pattern = pattern.clone().code() 
                       , prev = code )
                } ).collect::<Vec<_>>().join("\n");
            },
        }
    }

    code.parse().unwrap()
}