use crate::get_params::GetParams;

#[derive(Clone)]
pub struct GetMutParams {
    pub name: syn::Ident,
}

impl GetMutParams {
    pub fn parse(
        input: syn::parse::ParseStream,
        default_name: Option<syn::Ident>,
    ) -> syn::Result<Self> {
        let GetParams { name } = GetParams::parse(input, default_name)?;

        let name = syn::Ident::new(&format!("{}_mut", name), name.span());

        Ok(GetMutParams { name })
    }
}
