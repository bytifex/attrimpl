#[test]
fn tuple_struct_1() {
    #[attrimpl::attrimpl]
    struct TupleStruct(#[attrimpl(from, into, deref_mut)] String);

    // from String
    let mut value = TupleStruct::from("test".to_string());
    assert_eq!(value.0, "test");

    value.push_str("ing");
    assert_eq!(*value, "testing");

    // into String
    let s: String = value.into();
    assert_eq!(s, "testing");
}

#[test]
fn tuple_struct_access() {
    #[attrimpl::attrimpl]
    struct TupleStruct(
        #[attrimpl(
            get_ref(name = "a"),
            get_clone(name = "b"),
            access(name = "c"),
            access(name = "d", get_ref),
            access(name = "e", get_clone)
        )]
        String,
        #[attrimpl(
            get_ref(name = "f"),
            get_clone(name = "g"),
            get_copy(name = "h"),
            access(name = "i"),
            access(name = "j", get_ref),
            access(name = "k", get_clone),
            access(name = "l", get_copy)
        )]
        usize,
    );

    let mut value = TupleStruct("test".to_string(), 42);

    // String accessors
    // get_ref
    let _r: &String = value.a();

    // get_clone
    let _r: String = value.b();

    // access
    let _r: &String = value.c();
    let _r: &mut String = value.c_mut();

    // access with get_ref
    let _r: &String = value.d();
    let _r: &mut String = value.d_mut();

    // access with get_clone
    let _r: String = value.e();
    let _r: &mut String = value.e_mut();

    // usize accessors
    // get_ref
    let _r: &usize = value.f();

    // get_clone
    let _r: usize = value.g();

    // get_copy
    let _r: usize = value.h();

    // access
    let _r: &usize = value.i();
    let _r: &mut usize = value.i_mut();

    // access with get_ref
    let _r: &usize = value.j();
    let _r: &mut usize = value.j_mut();

    // access with get_clone
    let _r: usize = value.k();
    let _r: &mut usize = value.k_mut();

    // access with get_copy
    let _r: usize = value.l();
    let _r: &mut usize = value.l_mut();
}
