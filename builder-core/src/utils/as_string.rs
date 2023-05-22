
/// A workaround for implementing `to_string` like methods for custom types.
pub trait AsString {
    /// Returns the string representation of the item.
    fn as_string(&self) -> String;
}
