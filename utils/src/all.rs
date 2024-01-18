//! Provides a way for an attribute to provide "all" options

/// Provides the ability to return an iterator of all possible choices
pub trait AllStatic: Sized {
    /// Returns all possible choices
    fn all() -> impl Iterator<Item = Self>;
}
