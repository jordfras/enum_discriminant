use enum_discriminant::discriminant;

#[discriminant(unknown)]
enum EnumWithUnknownType {
    Unit = 17,
}

fn main() {}
