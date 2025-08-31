use crate::syn_field::SynField;

pub struct SynVariant {
    pub variant: syn::Variant,
    pub fields: Vec<SynField>,
}

impl SynVariant {
    pub fn parse(variant: &mut syn::Variant) -> syn::Result<Self> {
        Ok(SynVariant {
            fields: SynField::parse(&mut variant.fields)?,
            variant: variant.clone(),
        })
    }
}
