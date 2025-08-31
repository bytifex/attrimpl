#[test]
fn test_enum() {
    #[attrimpl::attrimpl]
    enum Enum {
        S(#[attrimpl(convert)] String),
        U8 {
            #[attrimpl(from)]
            byte: u8,
        },
        F64(#[attrimpl(convert)] f64),
    }

    {
        let value = Enum::from("test".to_string());
        match value {
            Enum::S(s) => assert_eq!(s, "test".to_string()),
            _ => panic!("expected Enum::S"),
        }
    }

    {
        let value = Box::<Enum>::from(42u8);
        match *value {
            Enum::U8 { byte } => assert_eq!(byte, 42u8),
            _ => panic!("expected Enum::U8"),
        }
    }

    {
        let value = Box::<Enum>::from(3.14f64);
        match *value {
            Enum::F64(f) => assert_eq!(f, 3.14f64),
            _ => panic!("expected Enum::F64"),
        }
    }
}
