#![allow(dead_code)]

#[test]
fn compilation() {
    #[attrimpl::attrimpl]
    enum Enum<'a, const N: usize = 7> {
        S(#[attrimpl(convert)] &'a String),
        U8 {
            #[attrimpl(from)]
            byte: u8,
        },
        F64(#[attrimpl(convert)] f64),
    }
}
