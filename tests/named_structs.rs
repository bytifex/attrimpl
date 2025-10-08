#[test]
fn named_struct_convert() {
    #[attrimpl::attrimpl]
    struct NamedStruct {
        #[attrimpl(from, into)]
        name: String,
    }

    // from
    let value = Box::<NamedStruct>::from("test".to_string());
    assert_eq!(value.name, "test");

    // into
    let value: String = (*value).into();
    assert_eq!(value, "test");
}

#[test]
fn named_struct_complex() {
    #[attrimpl::attrimpl]
    struct NamedStruct {
        #[attrimpl(into)]
        #[attrimpl(as_ref, as_mut)]
        #[attrimpl(get_ref)]
        name: String,

        #[attrimpl(deref_mut)]
        #[attrimpl(access)]
        hobby: String,
    }

    let mut value = NamedStruct {
        name: "Jane Doe".to_string(),
        hobby: "rock climbing".to_string(),
    };

    // deref_mut
    *value = "ice climbing".to_string();
    // deref
    assert_eq!(*value, "ice climbing");

    // get_mut
    *value.hobby_mut() = "rock climbing".to_string();
    // get_ref
    assert_eq!(value.hobby(), "rock climbing");

    // get_ref
    assert_eq!(value.name(), "Jane Doe");

    // as_ref
    assert_eq!(value.as_ref(), "Jane Doe");
    // as_mut
    *value.as_mut() = "John Doe".to_string();
    assert_eq!(value.as_ref(), "John Doe");

    // into
    let value: String = value.into();
    assert_eq!(value, "John Doe");
}
