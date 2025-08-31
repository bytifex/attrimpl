use syn::punctuated::Punctuated;

use crate::{MUTUALLY_EXCLUSIVE_DIRECTIVES, directive::Directive};

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
                        directive, excluded_by,
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

impl syn::parse::Parse for Directives {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let directives = Punctuated::<Directive, syn::Token![,]>::parse_terminated(input)?;

        Ok(Directives {
            directives: directives.into_iter().collect(),
        })
    }
}

fn excluded_by<'a>(directives: &'a [Directive], directive: &Directive) -> Option<&'a Directive> {
    // checking for dupications
    if let Some(directive) = directives.iter().find(|d| **d == *directive) {
        return Some(directive);
    }

    // checking for mutually-exclusive directives
    for (a, b) in MUTUALLY_EXCLUSIVE_DIRECTIVES.iter() {
        if let Some(directive) = directives
            .iter()
            .find(|d| (*directive == *a && **d == *b) || (*directive == *b && **d == *a))
        {
            return Some(directive);
        }
    }

    None
}
