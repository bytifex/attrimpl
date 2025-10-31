#[derive(Copy, Clone, Default)]
pub enum GetRefType {
    #[default]
    Ref,
    Copy,
    Clone,
}

#[derive(Clone)]
pub struct AccessParams {
    pub get_name: syn::Ident,
    pub get_mut_name: syn::Ident,
    pub get_ref_type: GetRefType,
}

impl AccessParams {
    pub fn parse(
        input: syn::parse::ParseStream,
        default_name: Option<syn::Ident>,
    ) -> syn::Result<Self> {
        fn set_get_ref_type(
            existing: &mut Option<GetRefType>,
            new: GetRefType,
            span: proc_macro2::Span,
        ) -> syn::Result<()> {
            if existing.is_some() {
                return Err(syn::Error::new(span, "get_ref_type already specified"));
            }
            *existing = Some(new);
            Ok(())
        }

        fn set_name(
            existing: &mut Option<syn::Ident>,
            new: syn::LitStr,
            span: proc_macro2::Span,
        ) -> syn::Result<()> {
            if existing.is_some() {
                return Err(syn::Error::new(span, "name already specified"));
            }
            *existing = Some(syn::Ident::new(&new.value(), new.span()));
            Ok(())
        }

        let mut name = None;
        let mut get_ref_type = None;

        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);

            while !content.is_empty() {
                let ident: syn::Ident = content.parse()?;

                match ident.to_string().as_str() {
                    "get_clone" => {
                        set_get_ref_type(&mut get_ref_type, GetRefType::Clone, ident.span())?;
                    }
                    "get_copy" => {
                        set_get_ref_type(&mut get_ref_type, GetRefType::Copy, ident.span())?;
                    }
                    "get_ref" => {
                        set_get_ref_type(&mut get_ref_type, GetRefType::Ref, ident.span())?;
                    }
                    "name" => {
                        content.parse::<syn::Token![=]>()?;
                        let name_lit: syn::LitStr = content.parse()?;
                        set_name(&mut name, name_lit, ident.span())?;
                    }
                    other => {
                        return Err(syn::Error::new(
                            ident.span(),
                            format!(
                                "expected one of `get_ref`, `get_copy`, `get_clone`, or `name = <..>`, found `{}`",
                                other
                            ),
                        ));
                    }
                }

                if content.is_empty() {
                    break;
                }

                content.parse::<syn::Token![,]>()?;
            }
        }

        let get_name = match name {
            Some(name) => name,
            None => default_name.ok_or_else(|| {
                syn::Error::new(
                    input.span(),
                    "name must be specified if no default name is provided",
                )
            })?,
        };
        let get_mut_name = syn::Ident::new(&format!("{}_mut", get_name), get_name.span());

        Ok(AccessParams {
            get_name,
            get_mut_name,
            get_ref_type: get_ref_type.unwrap_or_default(),
        })
    }
}
