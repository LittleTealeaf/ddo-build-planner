//! Provides a way for an attribute to provide "all" options

/// Returns the list of static possible choices for the structure
pub trait StaticOptions: Sized {
    /// Returns all possible choices
    fn get_static() -> impl Iterator<Item = Self>;
}
