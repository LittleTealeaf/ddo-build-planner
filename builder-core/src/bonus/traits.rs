use super::Bonus;

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
