#[derive(Clone)]
pub struct GetParams {
    pub name: syn::Ident,
}

impl GetParams {
    pub fn parse(
        input: syn::parse::ParseStream,
        default_name: Option<syn::Ident>,
    ) -> syn::Result<Self> {
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

        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);

            while !content.is_empty() {
                let ident: syn::Ident = content.parse()?;

                match ident.to_string().as_str() {
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

        let name = match name {
            Some(name) => name,
            None => default_name.ok_or_else(|| {
                syn::Error::new(
                    input.span(),
                    "name must be specified if no default name is provided",
                )
            })?,
        };

        Ok(GetParams { name })
    }
}
