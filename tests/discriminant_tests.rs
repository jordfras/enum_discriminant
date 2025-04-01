use enum_discriminant::discriminant;

#[discriminant(u8)]
#[derive(Debug, PartialEq)]
enum AllVariantTypes {
    Unit = 17,
    #[allow(dead_code)]
    Tuple(String) = 42,
    #[allow(dead_code)]
    Struct {
        x: u32,
        y: u32,
    } = 127,
}

#[test]
fn discrimnant_returns_correct_value_for_all_types_of_variants() {
    assert_eq!(17, AllVariantTypes::Unit.discriminant());
    assert_eq!(
        42,
        AllVariantTypes::Tuple("hello".to_string()).discriminant()
    );
    assert_eq!(127, AllVariantTypes::Struct { x: 17, y: 42 }.discriminant());
}

#[test]
fn can_create_unit_variant_from_discrimnant() {
    assert_eq!(
        Some(AllVariantTypes::Unit),
        AllVariantTypes::from_discriminant(17)
    );
    assert_eq!(None, AllVariantTypes::from_discriminant(42));
    assert_eq!(None, AllVariantTypes::from_discriminant(127));
}

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
}
