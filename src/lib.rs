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
    ("deref", "deref_mut"),
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

fn create_generic_idents(generics: &syn::Generics) -> syn::Generics {
    syn::Generics {
        params: generics
            .params
            .iter()
            .map(|param| match param {
                syn::GenericParam::Lifetime(lifetime_param) => {
                    syn::GenericParam::Lifetime(syn::LifetimeParam {
                        attrs: vec![],
                        lifetime: lifetime_param.lifetime.clone(),
                        colon_token: None,
                        bounds: Punctuated::default(),
                    })
                }
                syn::GenericParam::Type(type_param) => syn::GenericParam::Type(syn::TypeParam {
                    attrs: vec![],
                    ident: type_param.ident.clone(),
                    colon_token: None,
                    bounds: Punctuated::default(),
                    eq_token: None,
                    default: None,
                }),
                syn::GenericParam::Const(const_param) => syn::GenericParam::Type(syn::TypeParam {
                    attrs: vec![],
                    ident: const_param.ident.clone(),
                    colon_token: None,
                    bounds: Punctuated::default(),
                    eq_token: None,
                    default: None,
                }),
            })
            .collect(),
        ..Default::default()
    }
}

fn create_generics_for_impl(generics: &syn::Generics) -> syn::Generics {
    let mut generics = generics.clone();
    for param in &mut generics.params {
        match param {
            syn::GenericParam::Lifetime(_) => continue,
            syn::GenericParam::Type(type_param) => {
                type_param.eq_token = None;
                type_param.default = None;
            }
            syn::GenericParam::Const(const_param) => {
                const_param.eq_token = None;
                const_param.default = None;
            }
        }
        if let syn::GenericParam::Type(type_param) = param {
            type_param.bounds.clear();
        }
    }
    generics
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
