# enum_discriminant
Procedural macro for enum types to convert between variant and discriminant and vice versa.

# Added functions
When the procedural macro is used on an enum, you can get the discriminant, i.e., the numeric value
or ordinal, of a variant with the function `discrimnant()`. The other way around, variants can be
created with the function `from_discrimnant()`.

```rust
use enum_discriminant::discriminant;

#[discriminant(u8)]
#[derive(Debug, PartialEq)]
enum MyEnum {
    Zero,
    Two = 2,
    Three
}

assert_eq!(0, MyEnum::Zero.discriminant());
assert_eq!(2, MyEnum::Two.discriminant());
assert_eq!(3, MyEnum::Three.discriminant());

assert_eq!(Some(MyEnum::Zero), MyEnum::from_discriminant(0));
assert_eq!(None, MyEnum::from_discriminant(1));
assert_eq!(Some(MyEnum::Two), MyEnum::from_discriminant(2));
assert_eq!(Some(MyEnum::Three), MyEnum::from_discriminant(3));

```

The macro requires you to specify an integer type and will behave the same as `#[repr()]`.

It is possible to get the discriminant of all types of enum variants, including tuples and struct
variants. However, tuple and struct variants cannot be created from the discriminant since it
also requires the member values of the variant.

```rust
use enum_discriminant::discriminant;

#[discriminant(u8)]
#[derive(Debug, PartialEq)]
enum MyEnum {
    Unit,
    Tuple(u8, String),
    Struct{ a: u8, b: String }
}

assert_eq!(0, MyEnum::Unit.discriminant());
assert_eq!(1, MyEnum::Tuple(17, "blargh".to_string()).discriminant());
assert_eq!(2, MyEnum::Struct{ a: 17, b: "blargh".to_string() }.discriminant());

assert_eq!(Some(MyEnum::Unit), MyEnum::from_discriminant(0));
assert_eq!(None, MyEnum::from_discriminant(1));
assert_eq!(None, MyEnum::from_discriminant(2));

```

# Alternatives
There are similar, popular crates, including:

- [enum-ordinalize](https://crates.io/crates/enum-ordinalize)
- [enum-repr](https://crates.io/crates/enum-repr)
- [num_enum](https://crates.io/crates/num_enum)

All of these alternatives only support unit type enum variants. There are likely many other
similar crates with overlapping or identical functionality as this one.
