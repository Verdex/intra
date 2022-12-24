
use proc_macro::*;

use crate::data::*;

fn gen_atom_elements<'a>( elements : Vec<AtomElement<'a>> ) -> Vec<TokenTree> {
    vec![]
}

pub fn gen_atom<'a>( input : Atom<'a> ) -> TokenStream {
    let init = input.init.trees().into_iter().map(|x| x.clone()).collect::<Vec<_>>();
    let matcher = gen_atom_elements(input.seq);
    let resolve = input.resolve.trees().into_iter().map(|x| x.clone()).collect::<Vec<_>>();

    vec![init, matcher, resolve].into_iter().flatten().collect()
}