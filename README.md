# Warning
This package is under construction, handle with care!


# attrimpl
The aim of the package is to reduce boilerplate code by adding implementations in trivial cases.


## Directives
Directives that can be added before fields
- `from`: implements `From` trait for the given type
- `into`: implements `Into` trait for the given type
- `convert`: adds both `from` and `into` directives for the given field
- `deref`: implements `Deref` trait for the given type
- `deref_mut`: implements `Deref` and `DerefMut` traits for the given type
- `as_ref`: implements `AsRef` trait for the given type
- `as_mut`: implements `AsMut` trait for the given type
- `as`: adds both `as_ref` and `as_mut` directives for the given field
- `get_ref`: adds a getter method for the field
- `get_mut`: adds a mutable getter method for the field
- `get`: adds both `get_ref` and `get_mut` directives for the given field

[Naming convention for the getter methods](https://rust-lang.github.io/api-guidelines/naming.html#getter-names-follow-rust-convention-c-getter)


## Debugging
If the `debug` argument is added to the macro, then the generated code will be printed to stderr during compilation. Example:
```rust
#[attrimpl::attrimpl(debug)]
struct NamedStruct {
    #[attrimpl(from, into, deref_mut)]
    name: String,
}
```


## Examples
**Named struct (from, into, deref_mut):**
```rust
#[attrimpl::attrimpl]
struct NamedStruct {
    #[attrimpl(from, into, deref_mut)]
    name: String,
}

// non-boxed
let mut value = NamedStruct::from("test".to_string());
value.push_str("ing");
let value: String = value.into();

// boxed
let mut value = Box::<NamedStruct>::from("test".to_string());
value.push_str("ing");
let value: String = (*value).into();
```

**Named struct (into, as_ref, as_mut, get_ref, get_mut):**
```rust
#[attrimpl::attrimpl]
struct NamedStruct {
    #[attrimpl(into)]
    #[attrimpl(as_ref, as_mut)]
    #[attrimpl(get_ref)]
    name: String,

    #[attrimpl(deref_mut)]
    #[attrimpl(get)]
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
```

**Tuple struct:**
```rust
#[attrimpl::attrimpl]
struct TupleStruct1(#[attrimpl(from, into, deref_mut)] String);

// non-boxed
let mut value = TupleStruct1::from("test".to_string());
value.push_str("ing");
let value: String = value.into();

// boxed
let mut value = Box::<TupleStruct1>::from("test".to_string());
value.push_str("ing");
let value: String = (*value).into();
```

**Enum:**
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
* examine whether it is possible to implement deref, deref_mut, into, as_ref, as_mut on enums if every variant contains the same type
* `#[attrimpl(from)]` should work with multiple field structs and enums. Should be able to set the default values of the other fields (e.g., `#[attrimpl(from(field_default | container_default))]`).
* write test framework for compile time errors
* write a failing test where non-defined directive is given
* implement the following directives
  * `#[access(copy | clone | ref)]`
    * `#[attrimpl(get_ref)]`
    * `#[attrimpl(get_mut)]`
    * `#[attrimpl(get_copy)]`
    * `#[attrimpl(get_clone)]`
  * `#[attrimpl(display("asdasd {}"))]`
  * not sure whether to implement that one: #[attrimpl(borrow)]
  * search for othe possibilities of useful directives
