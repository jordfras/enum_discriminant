/// Trait for converting enum variants to their discriminant values. Added to enums with
/// the enum_discriminant procedural macro.
pub trait IntoDiscriminant {
    type DiscriminantType;

    /// Returns the discriminant numeric value of an enum variant.
    fn discriminant(&self) -> Self::DiscriminantType;
}
