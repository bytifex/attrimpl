# Warning
This package is under construction, handle with care!


# attrimpl
The aim of the package is to reduce boilerplate code by adding implementations in trivial cases:
- Traits
  - `Into`
  - `From`
  - `Deref`
  - `DerefMut`
  - `Asref`
  - `AsMut`
- Getter methods

## Examples
Named struct:
```rust
#[attrimpl::attrimpl]
struct NamedStruct {
    #[attrimpl(from, into)]
    name: String,
}

// non-boxed
let value = NamedStruct::from("test".to_string());
let value: String = value.into();

// boxed
let value = Box::<NamedStruct>::from("test".to_string());
let value: String = (*value).into();
```

Tuple struct:
```rust
#[attrimpl::attrimpl]
struct TupleStruct(#[attrimpl(from, into)] String);

// non-boxed
let value = TupleStruct::from("test".to_string());
let value: String = value.into();

// boxed
let value = Box::<TupleStruct>::from("test".to_string());
let value: String = (*value).into();
```

Enum:
```rust
#[attrimpl::attrimpl]
enum Enum {
    S(#[attrimpl(convert)] String),
    U8 {
        #[attrimpl(from)]
        byte: u8,
    },
    F64(#[attrimpl(convert)] f64),
}

// non-boxed
let value = Enum::from("test".to_string());

// boxed
let value = Box::<Enum>::from("test".to_string());
```


## Todo
* handle errors in the package instead of relying on the Rust compiler where possible
* write a proper readme file with examples
* `#[attrimpl(from)]` has to work with enums
* `#[attrimpl(from)]` should work with multiple field structs and enums. Should be able to set the default values of the other fields (e.g., `#[attrimpl(from(field_default | container_default))]`).
* write test framework for compile time errors
* write a failing test where non-defined directive is given
* implement the following directives
  * `#[attrimpl(convert)]`
    * `#[attrimpl(from)]`
    * `#[attrimpl(into)]`
  * `#[attrimpl(deref_both)]`
    * `#[attrimpl(deref)]`
    * `#[attrimpl(deref_mut)]`
  * `#[access(copy | clone | ref)]`
    * `#[attrimpl(get_ref)]`
    * `#[attrimpl(get_mut)]`
    * `#[attrimpl(get_copy)]`
    * `#[attrimpl(get_clone)]`
  * `#[attrimpl(as)]`
    * `#[attrimpl(as_ref)]`
    * `#[attrimpl(as_mut)]`
  * `#[attrimpl(display("asdasd {}"))]`
  * not sure whether to implement that one: #[attrimpl(borrow)]
  * search for othe possibilities of useful directives
