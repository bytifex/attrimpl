pub struct Args {
    debug: bool,
}

impl Args {
    pub fn debug(&self) -> bool {
        self.debug
    }
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Option<syn::Ident> = input.parse()?;

        let debug = if let Some(ident) = ident {
            if ident == "debug" {
                true
            } else {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("unknown argument: {}", ident),
                ));
            }
        } else {
            false
        };

        Ok(Args { debug })
    }
}
