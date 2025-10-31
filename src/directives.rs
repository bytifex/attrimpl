use syn::punctuated::Punctuated;

use crate::directive::Directive;

#[derive(Default)]
pub struct Directives {
    directives: Vec<Directive>,
}

impl Directives {
    pub fn extend_from(&mut self, other: Self) -> syn::Result<()> {
        for directive in other.directives {
            if let Some(excluded_by) = excluded_by(&self.directives, &directive) {
                return Err(syn::Error::new(
                    directive.span(),
                    format!(
                        "directives `{}` and `{}` are mutually exclusive",
                        directive.kind, excluded_by.kind,
                    ),
                ));
            }

            self.directives.push(directive);
        }

        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Directive> {
        self.directives.iter()
    }
}

impl Directives {
    pub fn parse(
        input: syn::parse::ParseStream,
        default_name: Option<syn::Ident>,
    ) -> syn::Result<Self> {
        // the code below is copied from syn::punctuated::Punctuated::parse_terminated
        let mut directives = Punctuated::new();

        loop {
            if input.is_empty() {
                break;
            }
            let value = Directive::parse(input, default_name.clone())?;
            directives.push_value(value);
            if input.is_empty() {
                break;
            }
            let punct = input.parse::<syn::Token![,]>()?;
            directives.push_punct(punct);
        }

        Ok(Directives {
            directives: directives.into_iter().collect(),
        })
    }
}

fn excluded_by<'a>(directives: &'a [Directive], directive: &Directive) -> Option<&'a Directive> {
    // checking for directive conflicts
    if let Some(directive) = directives
        .iter()
        .find(|d| d.kind.is_conflicted_with(&directive.kind))
    {
        return Some(directive);
    }

    None
}
