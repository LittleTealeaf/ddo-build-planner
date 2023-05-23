use crate::bonus::Bonus;

/// Implements the ability for one value to a list of other values.
///
/// This is specifically used when implementing attributes with the `All` type, instead of adding every single attribute individually.
pub trait GetCloned<T = Self> {
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

/// Implements the ability to get bonuses based on the attribute's current value.
///
/// The generic type is used to differentiate between some types that may implement different [`GetBonuses`] for different attributes.
pub trait GetBonuses<T = ()> {
    /// Gets a list of bonuses based on the attribute value.
    ///
    /// Returns `Some(Vec<Bonus>)` if the attribute has bonuses associated with it. Otherwise returns `None`.
    ///
    /// All bonuses from the attribute should have itself as a source. See [`BonusSource::Attribute`]
    ///
    /// [`BonusSource::Attribute`]: crate::bonus::BonusSource::Attribute
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>>;
}
