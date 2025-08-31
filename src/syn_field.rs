use crate::{ATTRIBUTE_NAME, are_path_segments_equal, directives::Directives};

pub struct SynField {
    pub index: usize,
    pub field: syn::Field,
    pub directives: Directives,
}

impl SynField {
    pub fn parse(fields: &mut syn::Fields) -> syn::Result<Vec<Self>> {
        let mut result_fields = Vec::new();

        for (field_index, field) in fields.iter_mut().enumerate() {
            let mut directives = Directives::default();

            let mut i = 0;
            while i < field.attrs.len() {
                let attr = &field.attrs[i];

                if let syn::Meta::List(attr) = &attr.meta {
                    if are_path_segments_equal(&attr.path.segments, &[ATTRIBUTE_NAME]) {
                        let tmp: Directives = syn::parse2(attr.tokens.clone())?;
                        directives.extend_from(tmp)?;

                        field.attrs.swap_remove(i);

                        continue;
                    } else if are_path_segments_equal(
                        &attr.path.segments,
                        &[ATTRIBUTE_NAME, "display"],
                    ) {
                        field.attrs.swap_remove(i);

                        continue;
                    }
                }

                i += 1;
            }

            result_fields.push(SynField {
                index: field_index,
                field: field.clone(),
                directives,
            });
        }

        Ok(result_fields)
    }
}
