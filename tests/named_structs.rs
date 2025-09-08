#[test]
fn named_struct_test() {
    #[attrimpl::attrimpl]
    struct NamedStruct {
        #[attrimpl(from, into)]
        // #[attrimpl(deref_both)]
        // #[attrimpl(as_ref, as_mut)]
        name: String,
    }

    let value = Box::<NamedStruct>::from("test".to_string());
    assert_eq!(value.name, "test");

    let value: String = (*value).into();
    assert_eq!(value, "test");
}
