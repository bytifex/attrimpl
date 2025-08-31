mod args;
mod directive;
mod directives;
mod item;
mod syn_field;
mod syn_item_enum;
mod syn_item_struct;
mod syn_variant;

use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token};

use crate::{args::Args, item::Item};

const ATTRIBUTE_NAME: &str = "attrimpl";

const MUTUALLY_EXCLUSIVE_DIRECTIVES: &[(&str, &str)] = &[
    ("convert", "from"),
    ("convert", "into"),
    ("deref_both", "deref"),
    ("deref_both", "deref_mut"),
    ("access", "get_ref"),
    ("access", "get_clone"),
    ("access", "get_mut"),
    ("access", "get_copy"),
    ("as", "as_ref"),
    ("as", "as_mut"),
];

fn are_path_segments_equal(
    punctuated: &Punctuated<syn::PathSegment, token::PathSep>,
    path_segments: &[&str],
) -> bool {
    if punctuated.len() != path_segments.len() {
        return false;
    }

    punctuated
        .iter()
        .zip(path_segments.iter())
        .all(|(seg, ident)| seg.ident == ident)
}

#[proc_macro_attribute]
pub fn attrimpl(arg: TokenStream, input: TokenStream) -> TokenStream {
    let args: Args = syn::parse_macro_input!(arg);

    let item: Item = syn::parse_macro_input!(input);

    let tokens = quote! {
        #item
    };

    if args.debug() {
        eprintln!("{}", tokens);
    }

    tokens.into()
}
