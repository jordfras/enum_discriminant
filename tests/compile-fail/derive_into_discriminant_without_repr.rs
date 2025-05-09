use enum_discriminant::IntoDiscriminant;

#[derive(IntoDiscriminant)]
enum MyEnum {
    Variant,
}

fn main() {}
