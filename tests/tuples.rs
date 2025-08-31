// #[attrimpl::attrimpl]
// struct Tuple2(
//     #[attrimpl(from, into)] String,
//     #[attrimpl(convert)] i32,
// );

// #[attrimpl::attrimpl]
// struct Tuple3(String, f64, #[attrimpl(from, into)] String);

// #[attrimpl::attrimpl]
// struct Tuple3Err(
//     #[attrimpl(from, into)] String,
//     #[attrimpl(convert)] f64,
//     #[attrimpl(from, into)] String,
// );

#[test]
fn tuple1() {
    #[attrimpl::attrimpl]
    struct Tuple1(#[attrimpl(from, into)] String);

    let s = Tuple1::from("test".to_string());
    assert_eq!(s.0, "test".to_string());

    let s: String = s.into();
    assert_eq!(s, "test");
}
