use quote::quote;

use crate::{directive::Directive, syn_variant::SynVariant};

pub struct SynItemEnum {
    item_enum: syn::ItemEnum,
    variants: Vec<SynVariant>,
}

impl SynItemEnum {
    pub fn parse(mut item_enum: syn::ItemEnum) -> syn::Result<Self> {
        Ok(SynItemEnum {
            variants: item_enum
                .variants
                .iter_mut()
                .map(SynVariant::parse)
                .collect::<Result<Vec<_>, _>>()?,
            item_enum,
        })
    }
}

impl quote::ToTokens for SynItemEnum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let item_enum = &self.item_enum;

        tokens.extend(quote! {
            #item_enum
        });

        for variant in &self.variants {
            for field in &variant.fields {
                for directive in field.directives.iter() {
                    directive_to_tokens(
                        item_enum,
                        variant,
                        &field.field,
                        field.index,
                        directive,
                        tokens,
                    );
                }
            }
        }
    }
}

fn directive_to_tokens(
    item_enum: &syn::ItemEnum,
    variant: &SynVariant,
    field: &syn::Field,
    field_index: usize,
    directive: &Directive,
    tokens: &mut proc_macro2::TokenStream,
) {
    #[allow(clippy::if_same_then_else)]
    if *directive == "from" {
        from_to_tokens(item_enum, variant, field, field_index, tokens);
    } else if *directive == "convert" {
        from_to_tokens(item_enum, variant, field, field_index, tokens);
    } else {
        panic!("unsupported directive for enum, directive = {}", directive);
    }
}

fn from_to_tokens(
    item_enum: &syn::ItemEnum,
    variant: &SynVariant,
    field: &syn::Field,
    _field_index: usize,
    tokens: &mut proc_macro2::TokenStream,
) {
    let ident = &item_enum.ident;
    let variant_ident = &variant.variant.ident;
    let generic_params = &item_enum.generics.params;
    let where_clause = item_enum.generics.where_clause.as_ref();
    let field_type = &field.ty;

    tokens.extend(if let Some(field_ident) = &field.ident {
        // it is a struct with named fields
        quote! {
            impl<#generic_params> ::core::convert::From<#field_type> for #ident
            #where_clause {
                fn from(value: #field_type) -> Self {
                    Self::#variant_ident {
                        #field_ident: value,
                    }
                }
            }
        }
    } else {
        // it is a tuple struct
        quote! {
            impl<#generic_params> ::core::convert::From<#field_type> for #ident
            #where_clause {
                fn from(value: #field_type) -> Self {
                    Self::#variant_ident(value)
                }
            }
        }
    });

    tokens.extend(quote! {
        impl<#generic_params> ::core::convert::From<#field_type> for ::std::boxed::Box<#ident>
        #where_clause {
            fn from(value: #field_type) -> Self {
                ::std::boxed::Box::new(#ident::from(value))
            }
        }
    });
}
