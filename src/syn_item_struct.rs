use quote::quote;

use crate::{
    create_generic_idents, create_generics_for_impl, directive::Directive, syn_field::SynField,
};

pub struct SynItemStruct {
    item_struct: syn::ItemStruct,
    fields: Vec<SynField>,
}

impl SynItemStruct {
    pub fn parse(mut item_struct: syn::ItemStruct) -> syn::Result<Self> {
        Ok(SynItemStruct {
            fields: SynField::parse(&mut item_struct.fields)?,
            item_struct,
        })
    }
}

impl quote::ToTokens for SynItemStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let item_struct = &self.item_struct;

        tokens.extend(quote! {
            #item_struct
        });

        for field in &self.fields {
            for directive in field.directives.iter() {
                directive_to_tokens(item_struct, &field.field, field.index, directive, tokens);
            }
        }
    }
}

fn directive_to_tokens(
    item_struct: &syn::ItemStruct,
    field: &syn::Field,
    field_index: usize,
    directive: &Directive,
    tokens: &mut proc_macro2::TokenStream,
) {
    let generics_for_impl = create_generics_for_impl(&item_struct.generics);
    let generic_idents = create_generic_idents(&item_struct.generics);

    if *directive == "from" {
        from_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
    } else if *directive == "into" {
        into_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
    } else if *directive == "convert" {
        from_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
        into_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
    } else if *directive == "deref" {
        deref_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
    } else if *directive == "deref_mut" {
        deref_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
        deref_mut_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
    } else if *directive == "as_ref" {
        as_ref_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
    } else if *directive == "as_mut" {
        as_mut_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
    } else if *directive == "as" {
        as_ref_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
        as_mut_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
    } else if *directive == "get_ref" {
        get_ref_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
    } else if *directive == "get_mut" {
        get_mut_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
    } else if *directive == "access" {
        get_ref_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
        get_mut_to_tokens(
            &generics_for_impl,
            &generic_idents,
            item_struct,
            field,
            field_index,
            tokens,
        );
    } else {
        panic!(
            "unsupported directive for struct, directive = {}",
            directive
        );
    }
}

fn from_to_tokens(
    generics_for_impl: &syn::Generics,
    generic_idents: &syn::Generics,
    item_struct: &syn::ItemStruct,
    field: &syn::Field,
    _field_index: usize,
    tokens: &mut proc_macro2::TokenStream,
) {
    let ident = &item_struct.ident;
    let where_clause = item_struct.generics.where_clause.as_ref();
    let field_type = &field.ty;

    tokens.extend(if let Some(field_ident) = &field.ident {
        // it is a struct with named fields
        quote! {
            impl #generics_for_impl ::core::convert::From<#field_type> for #ident #generic_idents
            #where_clause {
                fn from(value: #field_type) -> Self {
                    Self {
                        #field_ident: value,
                    }
                }
            }
        }
    } else {
        // it is a tuple struct
        quote! {
            impl #generics_for_impl ::core::convert::From<#field_type> for #ident #generic_idents
            #where_clause {
                fn from(value: #field_type) -> Self {
                    Self(value)
                }
            }
        }
    });

    tokens.extend(quote! {
        impl #generics_for_impl ::core::convert::From<#field_type> for ::std::boxed::Box<#ident #generic_idents>
        #where_clause {
            fn from(value: #field_type) -> Self {
                ::std::boxed::Box::new(#ident::from(value))
            }
        }
    });
}

fn into_to_tokens(
    generics_for_impl: &syn::Generics,
    generic_idents: &syn::Generics,
    item_struct: &syn::ItemStruct,
    field: &syn::Field,
    field_index: usize,
    tokens: &mut proc_macro2::TokenStream,
) {
    let ident = &item_struct.ident;
    let where_clause = item_struct.generics.where_clause.as_ref();
    let field_type = &field.ty;

    let field_reference_name = field
        .ident
        .as_ref()
        .map(|ident| quote! { #ident })
        .clone()
        .unwrap_or_else(|| {
            let field_index = syn::Index::from(field_index);
            quote! { #field_index }
        });

    tokens.extend(quote! {
        impl #generics_for_impl ::core::convert::Into<#field_type> for #ident #generic_idents
        #where_clause {
            fn into(self) -> #field_type {
                self.#field_reference_name
            }
        }
    });
}

fn deref_to_tokens(
    generics_for_impl: &syn::Generics,
    generic_idents: &syn::Generics,
    item_struct: &syn::ItemStruct,
    field: &syn::Field,
    field_index: usize,
    tokens: &mut proc_macro2::TokenStream,
) {
    let ident = &item_struct.ident;
    let where_clause = item_struct.generics.where_clause.as_ref();
    let field_type = &field.ty;

    let field_reference_name = field
        .ident
        .as_ref()
        .map(|ident| quote! { #ident })
        .clone()
        .unwrap_or_else(|| {
            let field_index = syn::Index::from(field_index);
            quote! { #field_index }
        });

    tokens.extend(quote! {
        impl #generics_for_impl ::core::ops::Deref for #ident #generic_idents
        #where_clause {
            type Target = #field_type;

            fn deref(&self) -> &#field_type {
                &self.#field_reference_name
            }
        }
    });
}

fn deref_mut_to_tokens(
    generics_for_impl: &syn::Generics,
    generic_idents: &syn::Generics,
    item_struct: &syn::ItemStruct,
    field: &syn::Field,
    field_index: usize,
    tokens: &mut proc_macro2::TokenStream,
) {
    let ident = &item_struct.ident;
    let where_clause = item_struct.generics.where_clause.as_ref();
    let field_type = &field.ty;

    let field_reference_name = field
        .ident
        .as_ref()
        .map(|ident| quote! { #ident })
        .clone()
        .unwrap_or_else(|| {
            let field_index = syn::Index::from(field_index);
            quote! { #field_index }
        });

    tokens.extend(quote! {
        impl #generics_for_impl ::core::ops::DerefMut for #ident #generic_idents
        #where_clause {
            fn deref_mut(&mut self) -> &mut #field_type {
                &mut self.#field_reference_name
            }
        }
    });
}

fn as_ref_to_tokens(
    generics_for_impl: &syn::Generics,
    generic_idents: &syn::Generics,
    item_struct: &syn::ItemStruct,
    field: &syn::Field,
    field_index: usize,
    tokens: &mut proc_macro2::TokenStream,
) {
    let ident = &item_struct.ident;
    let where_clause = item_struct.generics.where_clause.as_ref();
    let field_type = &field.ty;

    let field_reference_name = field
        .ident
        .as_ref()
        .map(|ident| quote! { #ident })
        .clone()
        .unwrap_or_else(|| {
            let field_index = syn::Index::from(field_index);
            quote! { #field_index }
        });

    tokens.extend(quote! {
        impl #generics_for_impl ::core::convert::AsRef<#field_type> for #ident #generic_idents
        #where_clause {
            fn as_ref(&self) -> &#field_type {
                &self.#field_reference_name
            }
        }
    });
}

fn as_mut_to_tokens(
    generics_for_impl: &syn::Generics,
    generic_idents: &syn::Generics,
    item_struct: &syn::ItemStruct,
    field: &syn::Field,
    field_index: usize,
    tokens: &mut proc_macro2::TokenStream,
) {
    let ident = &item_struct.ident;
    let where_clause = item_struct.generics.where_clause.as_ref();
    let field_type = &field.ty;

    let field_reference_name = field
        .ident
        .as_ref()
        .map(|ident| quote! { #ident })
        .clone()
        .unwrap_or_else(|| {
            let field_index = syn::Index::from(field_index);
            quote! { #field_index }
        });

    tokens.extend(quote! {
        impl #generics_for_impl ::core::convert::AsMut<#field_type> for #ident #generic_idents
        #where_clause {
            fn as_mut(&mut self) -> &mut #field_type {
                &mut self.#field_reference_name
            }
        }
    });
}

fn get_ref_to_tokens(
    generics_for_impl: &syn::Generics,
    generic_idents: &syn::Generics,
    item_struct: &syn::ItemStruct,
    field: &syn::Field,
    field_index: usize,
    tokens: &mut proc_macro2::TokenStream,
) {
    let ident = &item_struct.ident;
    let where_clause = item_struct.generics.where_clause.as_ref();
    let field_type = &field.ty;

    let Some(field_ident) = &field.ident else {
        panic!("get_ref and get_mut is accepted only on named structs");
    };

    // field_reference_name is kept here to allow introducing get aliases later
    let field_reference_name = field
        .ident
        .as_ref()
        .map(|ident| quote! { #ident })
        .clone()
        .unwrap_or_else(|| {
            let field_index = syn::Index::from(field_index);
            quote! { #field_index }
        });

    tokens.extend(quote! {
        impl #generics_for_impl #ident #generic_idents
        #where_clause {
            pub fn #field_ident(&self) -> &#field_type {
                &self.#field_reference_name
            }
        }
    });
}

fn get_mut_to_tokens(
    generics_for_impl: &syn::Generics,
    generic_idents: &syn::Generics,
    item_struct: &syn::ItemStruct,
    field: &syn::Field,
    field_index: usize,
    tokens: &mut proc_macro2::TokenStream,
) {
    let ident = &item_struct.ident;
    let where_clause = item_struct.generics.where_clause.as_ref();
    let field_type = &field.ty;

    let Some(field_ident) = &field.ident else {
        panic!("get_ref and get_mut is accepted only on named structs");
    };

    let field_ident = syn::Ident::new(&format!("{}_mut", field_ident), field_ident.span());

    // field_reference_name is kept here to allow introducing get aliases later
    let field_reference_name = field
        .ident
        .as_ref()
        .map(|ident| quote! { #ident })
        .clone()
        .unwrap_or_else(|| {
            let field_index = syn::Index::from(field_index);
            quote! { #field_index }
        });

    tokens.extend(quote! {
        impl #generics_for_impl #ident #generic_idents
        #where_clause {
            pub fn #field_ident(&mut self) -> &mut #field_type {
                &mut self.#field_reference_name
            }
        }
    });
}
