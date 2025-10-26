#[derive(Clone, PartialEq)]
pub enum DirectiveKind {
    From,
    Into,
    Convert,

    GetRef,
    GetMut,
    Access,

    AsRef,
    AsMut,
    As,

    Deref,
    DerefMut,
}

pub struct Directive {
    pub span: proc_macro2::Span,
    pub kind: DirectiveKind,
}

impl Directive {
    pub fn span(&self) -> proc_macro2::Span {
        self.span
    }
}

impl DirectiveKind {
    pub fn name(&self) -> &'static str {
        match self {
            DirectiveKind::From => "from",
            DirectiveKind::Into => "into",
            DirectiveKind::Convert => "convert",

            DirectiveKind::GetRef => "get_ref",
            DirectiveKind::GetMut => "get_mut",
            DirectiveKind::Access => "access",

            DirectiveKind::AsRef => "as_ref",
            DirectiveKind::AsMut => "as_mut",
            DirectiveKind::As => "as",

            DirectiveKind::Deref => "deref",
            DirectiveKind::DerefMut => "deref_mut",
        }
    }
}

impl std::fmt::Display for DirectiveKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl syn::parse::Parse for Directive {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        let kind = match ident.to_string().as_str() {
            "from" => DirectiveKind::From,
            "into" => DirectiveKind::Into,
            "convert" => DirectiveKind::Convert,

            "get_ref" => DirectiveKind::GetRef,
            "get_mut" => DirectiveKind::GetMut,
            "access" => DirectiveKind::Access,

            "as_ref" => DirectiveKind::AsRef,
            "as_mut" => DirectiveKind::AsMut,
            "as" => DirectiveKind::As,

            "deref" => DirectiveKind::Deref,
            "deref_mut" => DirectiveKind::DerefMut,

            _ => {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("unknown directive `{}`", ident),
                ));
            }
        };

        Ok(Directive {
            span: ident.span(),
            kind,
        })
    }
}
