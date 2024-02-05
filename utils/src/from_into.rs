//! Adds a trait that implements From for every item that has an Into

/// Adds the `from_into` trait for simple converting of into statements
pub trait FromInto<T> {
    /// Converts an object using the `.into()` parameter
    fn from_into(item: T) -> Self;
}

impl<A, B> FromInto<B> for A
where
    B: Into<A>,
{
    fn from_into(item: B) -> Self {
        item.into()
    }
}
