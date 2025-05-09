/// Trait for converting enum variants to their discriminant values. Added to enums with
/// the corresponding derive macro.
pub trait IntoDiscriminant {
    type DiscriminantType;

    /// Returns the discriminant numeric value of an enum variant.
    fn discriminant(&self) -> Self::DiscriminantType;
}

/// Trait for creating enum unit variants from their discriminant values. Added to enums
/// with the corresponding derive macro.
pub trait FromDiscriminant {
    type DiscriminantType;

    /// Creates an enum variant from its discriminant numeric value.
    fn from_discriminant(discriminant: Self::DiscriminantType) -> Option<Self>
    where
        Self: Sized;
}
