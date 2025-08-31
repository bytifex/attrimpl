#[derive(PartialEq)]
pub struct Directive {
    ident: syn::Ident,
}

impl Directive {
    pub fn span(&self) -> proc_macro2::Span {
        self.ident.span()
    }
}

impl std::fmt::Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.ident)
    }
}

impl syn::parse::Parse for Directive {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        Ok(Directive { ident })
    }
}

impl PartialEq<&str> for Directive {
    fn eq(&self, other: &&str) -> bool {
        self.ident == *other
    }
}
