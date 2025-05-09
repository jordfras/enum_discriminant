use enum_discriminant::FromDiscriminant;

#[derive(FromDiscriminant)]
enum MyEnum {
    Variant,
}

fn main() {}
