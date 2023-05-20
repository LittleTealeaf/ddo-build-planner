

/// Implements the ability for one value to a list of other values.
///
/// This is specifically used when implementing attributes with the `All` type, instead of adding every single attribute individually.
pub trait GetCloned<T> {
    /// Can return a list of values that the value splits into.
    ///
    /// Returns `None` if the attribute does not split into other attributes.
    ///
    /// By implementation in [Breakdowns][`Breakdowns`], the orriginal attribute *is not* removed, thus it is possible to get the value of the original attribute even if it breaks into others.
    ///
    ///
    /// [`Breakdowns`]: crate::breakdown::Breakdowns
    fn get_cloned(&self) -> Option<Vec<T>>;
}
