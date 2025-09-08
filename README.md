# Warning
This package is under construction, handle with care!


# attrimpl
The aim of the package is to reduce boilerplate code by adding implementations in trivial cases.

## Directives
Directives that can be added before fields
- `from`: implements `From` trait for the given type
- `into`: implements `Into` trait for the given type
- `convert`: add both `from` and `into` directives for the given field
- `deref`: implements `Deref` trait for the given type
- `deref_mut`: implements `Deref` and `DerefMut` traits for the given type

### Upcoming implementations:
- Traits
  - `Into`
  - `From`
  - `Deref`
  - `DerefMut`
  - `Asref`
  - `AsMut`
- Getter methods

## Examples
**Named struct:**
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
