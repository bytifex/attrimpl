#[test]
fn tuple_struct_1() {
    #[attrimpl::attrimpl]
    struct TupleStruct1(#[attrimpl(from, into, deref_mut)] String);

    // from String
    let mut value = TupleStruct1::from("test".to_string());
    assert_eq!(value.0, "test");

    value.push_str("ing");
    assert_eq!(*value, "testing");

    // into String
    let s: String = value.into();
    assert_eq!(s, "testing");
}
