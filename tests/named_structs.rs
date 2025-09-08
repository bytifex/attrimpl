#[test]
fn named_struct_test() {
    #[attrimpl::attrimpl]
    struct NamedStruct {
        #[attrimpl(from, into)]
        // #[attrimpl(as_ref, as_mut)]
        #[attrimpl(deref_mut)]
        name: String,
    }

    let mut value = Box::<NamedStruct>::from("test".to_string());
    assert_eq!(value.name, "test");

    value.push_str("ing");
    assert_eq!(**value, "testing".to_string());

    let value: String = (*value).into();
    assert_eq!(value, "testing");
}
