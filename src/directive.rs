use crate::{access_params::AccessParams, get_mut_params::GetMutParams, get_params::GetParams};

#[derive(Clone)]
pub enum DirectiveKind {
    From,
    Into,
    Convert,

    GetRef(GetParams),
    GetCopy(GetParams),
    GetClone(GetParams),
    GetMut(GetMutParams),
    Access(AccessParams),

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
    fn name(&self) -> &'static str {
        match self {
            DirectiveKind::From => "from",
            DirectiveKind::Into => "into",
            DirectiveKind::Convert => "convert",

            DirectiveKind::GetRef(_) => "get_ref",
            DirectiveKind::GetCopy(_) => "get_copy",
            DirectiveKind::GetClone(_) => "get_clone",
            DirectiveKind::GetMut(_) => "get_mut",
            DirectiveKind::Access(_) => "access",

            DirectiveKind::AsRef => "as_ref",
            DirectiveKind::AsMut => "as_mut",
            DirectiveKind::As => "as",

            DirectiveKind::Deref => "deref",
            DirectiveKind::DerefMut => "deref_mut",
        }
    }

    pub fn is_conflicted_with(&self, other: &DirectiveKind) -> bool {
        match self {
            DirectiveKind::From => match other {
                DirectiveKind::From | DirectiveKind::Convert => true,
                DirectiveKind::Into
                | DirectiveKind::GetRef(_)
                | DirectiveKind::GetCopy(_)
                | DirectiveKind::GetClone(_)
                | DirectiveKind::GetMut(_)
                | DirectiveKind::Access(_)
                | DirectiveKind::AsRef
                | DirectiveKind::AsMut
                | DirectiveKind::As
                | DirectiveKind::Deref
                | DirectiveKind::DerefMut => false,
            },
            DirectiveKind::Into => match other {
                DirectiveKind::Into | DirectiveKind::Convert => true,
                DirectiveKind::From
                | DirectiveKind::GetRef(_)
                | DirectiveKind::GetCopy(_)
                | DirectiveKind::GetClone(_)
                | DirectiveKind::GetMut(_)
                | DirectiveKind::Access(_)
                | DirectiveKind::AsRef
                | DirectiveKind::AsMut
                | DirectiveKind::As
                | DirectiveKind::Deref
                | DirectiveKind::DerefMut => false,
            },
            DirectiveKind::Convert => match other {
                DirectiveKind::From | DirectiveKind::Into | DirectiveKind::Convert => true,
                DirectiveKind::GetRef(_)
                | DirectiveKind::GetCopy(_)
                | DirectiveKind::GetClone(_)
                | DirectiveKind::GetMut(_)
                | DirectiveKind::Access(_)
                | DirectiveKind::AsRef
                | DirectiveKind::AsMut
                | DirectiveKind::As
                | DirectiveKind::Deref
                | DirectiveKind::DerefMut => false,
            },
            DirectiveKind::GetRef(GetParams { name })
            | DirectiveKind::GetCopy(GetParams { name })
            | DirectiveKind::GetClone(GetParams { name })
            | DirectiveKind::GetMut(GetMutParams { name }) => match other {
                DirectiveKind::GetRef(GetParams { name: other_name })
                | DirectiveKind::GetCopy(GetParams { name: other_name })
                | DirectiveKind::GetClone(GetParams { name: other_name })
                | DirectiveKind::GetMut(GetMutParams { name: other_name }) => *name == *other_name,
                DirectiveKind::Access(AccessParams {
                    get_name,
                    get_mut_name,
                    ..
                }) => *name == *get_name || *name == *get_mut_name,
                DirectiveKind::From
                | DirectiveKind::Into
                | DirectiveKind::Convert
                | DirectiveKind::AsRef
                | DirectiveKind::AsMut
                | DirectiveKind::As
                | DirectiveKind::Deref
                | DirectiveKind::DerefMut => false,
            },
            DirectiveKind::Access(params) => match other {
                DirectiveKind::GetRef(GetParams { name })
                | DirectiveKind::GetCopy(GetParams { name })
                | DirectiveKind::GetClone(GetParams { name })
                | DirectiveKind::GetMut(GetMutParams { name }) => {
                    params.get_name == *name || params.get_mut_name == *name
                }
                DirectiveKind::Access(AccessParams {
                    get_name,
                    get_mut_name,
                    ..
                }) => {
                    params.get_name == *get_name
                        || params.get_name == *get_mut_name
                        || params.get_mut_name == *get_name
                        || params.get_mut_name == *get_mut_name
                }
                DirectiveKind::From
                | DirectiveKind::Into
                | DirectiveKind::Convert
                | DirectiveKind::AsRef
                | DirectiveKind::AsMut
                | DirectiveKind::As
                | DirectiveKind::Deref
                | DirectiveKind::DerefMut => false,
            },
            DirectiveKind::AsRef => match other {
                DirectiveKind::AsRef | DirectiveKind::As => true,
                DirectiveKind::From
                | DirectiveKind::Into
                | DirectiveKind::Convert
                | DirectiveKind::GetRef(_)
                | DirectiveKind::GetCopy(_)
                | DirectiveKind::GetClone(_)
                | DirectiveKind::GetMut(_)
                | DirectiveKind::Access(_)
                | DirectiveKind::AsMut
                | DirectiveKind::Deref
                | DirectiveKind::DerefMut => false,
            },
            DirectiveKind::AsMut => match other {
                DirectiveKind::AsMut | DirectiveKind::As => true,
                DirectiveKind::From
                | DirectiveKind::Into
                | DirectiveKind::Convert
                | DirectiveKind::GetRef(_)
                | DirectiveKind::GetCopy(_)
                | DirectiveKind::GetClone(_)
                | DirectiveKind::GetMut(_)
                | DirectiveKind::Access(_)
                | DirectiveKind::AsRef
                | DirectiveKind::Deref
                | DirectiveKind::DerefMut => false,
            },
            DirectiveKind::As => match other {
                DirectiveKind::AsRef | DirectiveKind::AsMut | DirectiveKind::As => true,
                DirectiveKind::From
                | DirectiveKind::Into
                | DirectiveKind::Convert
                | DirectiveKind::GetRef(_)
                | DirectiveKind::GetCopy(_)
                | DirectiveKind::GetClone(_)
                | DirectiveKind::GetMut(_)
                | DirectiveKind::Access(_)
                | DirectiveKind::Deref
                | DirectiveKind::DerefMut => false,
            },
            DirectiveKind::Deref => match other {
                DirectiveKind::Deref => true,
                DirectiveKind::From
                | DirectiveKind::Into
                | DirectiveKind::Convert
                | DirectiveKind::GetRef(_)
                | DirectiveKind::GetCopy(_)
                | DirectiveKind::GetClone(_)
                | DirectiveKind::GetMut(_)
                | DirectiveKind::Access(_)
                | DirectiveKind::AsRef
                | DirectiveKind::AsMut
                | DirectiveKind::As
                | DirectiveKind::DerefMut => false,
            },
            DirectiveKind::DerefMut => match other {
                DirectiveKind::DerefMut => true,
                DirectiveKind::From
                | DirectiveKind::Into
                | DirectiveKind::Convert
                | DirectiveKind::GetRef(_)
                | DirectiveKind::GetCopy(_)
                | DirectiveKind::GetClone(_)
                | DirectiveKind::GetMut(_)
                | DirectiveKind::Access(_)
                | DirectiveKind::AsRef
                | DirectiveKind::AsMut
                | DirectiveKind::As
                | DirectiveKind::Deref => false,
            },
        }
    }
}

impl std::fmt::Display for DirectiveKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Directive {
    pub fn parse(
        input: syn::parse::ParseStream,
        default_name: Option<syn::Ident>,
    ) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        let kind = match ident.to_string().as_str() {
            "from" => DirectiveKind::From,
            "into" => DirectiveKind::Into,
            "convert" => DirectiveKind::Convert,

            "get_ref" => DirectiveKind::GetRef(GetParams::parse(input, default_name)?),
            "get_clone" => DirectiveKind::GetClone(GetParams::parse(input, default_name)?),
            "get_copy" => DirectiveKind::GetCopy(GetParams::parse(input, default_name)?),
            "get_mut" => DirectiveKind::GetMut(GetMutParams::parse(input, default_name)?),
            "access" => DirectiveKind::Access(AccessParams::parse(input, default_name)?),

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
