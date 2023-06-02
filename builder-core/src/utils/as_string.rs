/// A workaround for implementing `to_string` like methods for custom types.
#[deprecated = "Implement Display for a struct instead"]
pub trait AsString {
    /// Returns the string representation of the item.
    fn as_string(&self) -> String;
}
