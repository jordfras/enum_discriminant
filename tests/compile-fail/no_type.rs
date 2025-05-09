use enum_discriminant::discriminant;

#[discriminant()]
enum EnumWithNoType {
    Unit = 17,
}

fn main() {}
