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
fn auto_assigned_discrimnants_are_handled_correctly() {
    #[discriminant(u8)]
    #[derive(Debug, PartialEq)]
    enum MyEnum {
        Zero,
        Two = 2,
        Three,
        Five = 5,
        Tuple(String),
        Struct { x: u32, y: u32 },
    }

    assert_eq!(0, MyEnum::Zero.discriminant());
    assert_eq!(2, MyEnum::Two.discriminant());
    assert_eq!(3, MyEnum::Three.discriminant());
    assert_eq!(5, MyEnum::Five.discriminant());
    assert_eq!(6, MyEnum::Tuple("hello".to_string()).discriminant());
    assert_eq!(7, MyEnum::Struct { x: 17, y: 42 }.discriminant());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_discriminant(0));
    assert_eq!(Some(MyEnum::Two), MyEnum::from_discriminant(2));
    assert_eq!(Some(MyEnum::Three), MyEnum::from_discriminant(3));
    assert_eq!(Some(MyEnum::Five), MyEnum::from_discriminant(5));
}

// Not testing 128 bit types, since considered unstable
#[test]
fn all_types_work() {
    #[discriminant(u8)]
    #[derive(Debug, PartialEq)]
    enum EnumU8 {
        Unit = 17,
    }
    assert_eq!(17, EnumU8::Unit.discriminant());
    assert_eq!(Some(EnumU8::Unit), EnumU8::from_discriminant(17));

    #[discriminant(u16)]
    #[derive(Debug, PartialEq)]
    enum EnumU16 {
        Unit = 65000,
    }
    assert_eq!(65000, EnumU16::Unit.discriminant());
    assert_eq!(Some(EnumU16::Unit), EnumU16::from_discriminant(65000));

    #[discriminant(u32)]
    #[derive(Debug, PartialEq)]
    enum EnumU32 {
        Unit = 165000,
    }
    assert_eq!(165000, EnumU32::Unit.discriminant());
    assert_eq!(Some(EnumU32::Unit), EnumU32::from_discriminant(165000));

    #[discriminant(u64)]
    #[derive(Debug, PartialEq)]
    enum EnumU64 {
        Unit = 17,
    }
    assert_eq!(17, EnumU64::Unit.discriminant());
    assert_eq!(Some(EnumU64::Unit), EnumU64::from_discriminant(17));

    #[discriminant(usize)]
    #[derive(Debug, PartialEq)]
    enum EnumUsize {
        Unit = 17,
    }
    assert_eq!(17, EnumUsize::Unit.discriminant());
    assert_eq!(Some(EnumUsize::Unit), EnumUsize::from_discriminant(17));

    #[discriminant(i8)]
    #[derive(Debug, PartialEq)]
    enum EnumI8 {
        Unit = -17,
    }
    assert_eq!(-17, EnumI8::Unit.discriminant());
    assert_eq!(Some(EnumI8::Unit), EnumI8::from_discriminant(-17));

    #[discriminant(i16)]
    #[derive(Debug, PartialEq)]
    enum EnumI16 {
        Unit = -32000,
    }
    assert_eq!(-32000, EnumI16::Unit.discriminant());
    assert_eq!(Some(EnumI16::Unit), EnumI16::from_discriminant(-32000));

    #[discriminant(i32)]
    #[derive(Debug, PartialEq)]
    enum EnumI32 {
        Unit = -132000,
    }
    assert_eq!(-132000, EnumI32::Unit.discriminant());
    assert_eq!(Some(EnumI32::Unit), EnumI32::from_discriminant(-132000));

    #[discriminant(i64)]
    #[derive(Debug, PartialEq)]
    enum EnumI64 {
        Unit = -17,
    }
    assert_eq!(-17, EnumI64::Unit.discriminant());
    assert_eq!(Some(EnumI64::Unit), EnumI64::from_discriminant(-17));

    #[discriminant(isize)]
    #[derive(Debug, PartialEq)]
    enum EnumIsize {
        Unit = -17,
    }
    assert_eq!(-17, EnumIsize::Unit.discriminant());
    assert_eq!(Some(EnumIsize::Unit), EnumIsize::from_discriminant(-17));
}

#[test]
fn expressions_in_discrimnant_works_with_auto_assign() {
    #[discriminant(u8)]
    #[derive(Debug, PartialEq)]
    enum MyEnum {
        Seven = 4 + 3,
        Eight,
    }

    assert_eq!(7, MyEnum::Seven.discriminant());
    assert_eq!(8, MyEnum::Eight.discriminant());

    assert_eq!(Some(MyEnum::Seven), MyEnum::from_discriminant(7));
    assert_eq!(Some(MyEnum::Eight), MyEnum::from_discriminant(8));
}

#[test]
fn c_repr_enum_works() {
    #[discriminant(C, u32)]
    #[derive(Debug, PartialEq)]
    enum MyEnum {
        Unit,
        Tuple(u32),
    }

    assert_eq!(0, MyEnum::Unit.discriminant());
    assert_eq!(1, MyEnum::Tuple(17).discriminant());

    assert_eq!(Some(MyEnum::Unit), MyEnum::from_discriminant(0));
    assert_eq!(None, MyEnum::from_discriminant(1));
}

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
}
