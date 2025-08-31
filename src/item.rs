use syn::parse::discouraged::Speculative;

use crate::{syn_item_enum::SynItemEnum, syn_item_struct::SynItemStruct};

pub enum Item {
    Struct(SynItemStruct),
    Enum(SynItemEnum),
}

impl syn::parse::Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fork_struct = input.fork();
        let fork_enum = input.fork();

        if let Ok(item_struct) = fork_struct.parse::<syn::ItemStruct>() {
            input.advance_to(&fork_struct);
            Ok(Item::Struct(SynItemStruct::parse(item_struct)?))
        } else if let Ok(item_enum) = fork_enum.parse::<syn::ItemEnum>() {
            input.advance_to(&fork_enum);
            Ok(Item::Enum(SynItemEnum::parse(item_enum)?))
        } else {
            Err(syn::Error::new(
                input.span(),
                "expected a struct or an enum",
            ))
        }
    }
}

impl quote::ToTokens for Item {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Item::Struct(item_struct) => {
                item_struct.to_tokens(tokens);
            }
            Item::Enum(item_enum) => {
                item_enum.to_tokens(tokens);
            }
        }
    }
}
