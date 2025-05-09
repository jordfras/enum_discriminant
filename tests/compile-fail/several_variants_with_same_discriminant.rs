use enum_discriminant::discriminant;

#[discriminant(u8)]
enum SeveralVariantsWithSameDiscriminant {
    Variant1 = 1,
    Variant2 = 1,
}

// The same error should be produced with repr
#[repr(u8)]
enum SeveralVariantsWithSameDiscriminantRepr {
    Variant1 = 1,
    Variant2 = 1,
}

fn main() {}
