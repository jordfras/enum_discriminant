use enum_discriminant::discriminant;

#[discriminant(u8, u32)]
enum SeveralIntegerTypes {
    Variant,
}

// The same error should be produced with repr
#[repr(u8, u32)]
enum SeveralIntegerTypesRepr {
    Variant,
}

fn main() {}
