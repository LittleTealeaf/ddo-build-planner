//! Provides a way for an attribute to provide "all" options

/// Returns the list of static possible choices for the structure
pub trait StaticValues: Sized {
    /// Returns all possible choices
    fn values() -> impl Iterator<Item = Self>;
}
