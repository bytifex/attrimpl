#[test]
fn named_struct_convert() {
    #[attrimpl::attrimpl]
    struct NamedStruct {
        #[attrimpl(from, into)]
        name: String,
    }

    // from String
    let value = Box::<NamedStruct>::from("test".to_string());
    assert_eq!(value.name, "test");

    // Boxed from String
    let value = Box::<NamedStruct>::from("test".to_string());
    assert_eq!(value.name, "test");

    // into String
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

    // into from String
    let value: String = value.into();
    assert_eq!(value, "John Doe");
}

#[test]
fn named_struct_access() {
    #[attrimpl::attrimpl]
    struct NamedStruct {
        #[attrimpl(
            get_ref,
            get_mut,
            get_ref(name = "a"),
            get_clone(name = "b"),
            access(name = "c"),
            access(name = "d", get_ref),
            access(name = "e", get_clone)
        )]
        name: String,

        #[attrimpl(
            get_ref,
            get_mut,
            get_ref(name = "f"),
            get_clone(name = "g"),
            get_copy(name = "h"),
            access(name = "i"),
            access(name = "j", get_ref),
            access(name = "k", get_clone),
            access(name = "l", get_copy)
        )]
        value: usize,
    }

    let mut value = NamedStruct {
        name: "test".to_string(),
        value: 42,
    };

    // String accessors
    // get_ref without parameters
    let _r: &String = value.name();

    // get_mut without parameters
    let _r: &mut String = value.name_mut();

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
    // get_ref without parameters
    let _r: &usize = value.value();

    // get_mut without parameters
    let _r: &mut usize = value.value_mut();

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
