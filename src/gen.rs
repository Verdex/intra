
use proc_macro::*;

use crate::data::*;

macro_rules! tree_to_string {
    ($x:expr) => {
        $x.trees().into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join("")
    };
}

fn use_pre_map(map : Option<IntraIdent>, ident : String) -> String {
    match map {
        Some(map) => format!( "{}({})", tree_to_string!(map), ident ),
        None => ident,
    }
}

pub fn gen_atom( input : Atom ) -> TokenStream {
    let init = tree_to_string!(input.init);
    let mut elements = input.seq;
    let resolve = tree_to_string!(input.resolve);

    let mut previous = std::iter::once(vec![init])
        .chain(elements.iter()
                       .flat_map(|x| match x { AtomElement::Pattern { next, .. } => vec![next], _ => vec![] })
                       .map(|x| x.clone().into_iter().map(|y| tree_to_string!(y)).collect::<Vec<_>>())
              )
        .collect::<Vec<_>>();
    previous.pop();

    let mut code = resolve; 

    while elements.len() != 0 {
        match elements.pop().unwrap() {
            AtomElement::Execute(e) => { code = format!("{}\n{}", tree_to_string!(e), code) },
            AtomElement::Pattern { pre_map, pattern, .. } => {
                code = previous.pop().unwrap().into_iter().map(|ident| {
                    format!("match {ident} {{
                        {pattern} => {{
                            {prev}
                        }},
                        _ => {{ }},
                    }}", ident = use_pre_map(pre_map.clone(), ident)
                       , pattern = tree_to_string!(pattern.clone()) 
                       , prev = code )
                } ).collect::<Vec<_>>().join("\n");
            },
        }
    }

    code.parse().unwrap()
}