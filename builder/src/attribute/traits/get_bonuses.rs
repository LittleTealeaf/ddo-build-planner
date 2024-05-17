use rust_decimal::Decimal;

use crate::bonus::BonusTemplate;

/// Implements the ability to get bonuses from different [`Attribute`] sub-types.
///
/// The generic type `T` is used as a means of differentiating different bonuses for an
/// [`Attribute`] sub-type that has multiple instances. `T` is intended to be a 0-size struct. An
/// example of this is with [`Ability`], which has the two helper structs [`_AbilityScore`] and
/// [`_AbilityModifier`].
///
/// # Examples
///
/// ```
/// use builder::{bonus::BonusTemplate, attribute::GetBonuses};
/// use rust_decimal::Decimal;
///
/// enum Test {
///     A,
///     B,
/// }
///
/// impl GetBonuses for Test {
///     fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
///         // implementation
///         None // return None if there are none
///     }
/// }
///
/// let value = Test::A.get_bonuses(Decimal::from(10));
/// assert!(value.is_none());
///
/// ```
///
/// If there are no bonuses for a type, [`None`] is returned.
///
/// ```
/// use builder::{bonus::BonusTemplate, attribute::GetBonuses};
/// use rust_decimal::Decimal;
///
/// enum Test {
///     A,
///     B
/// }
///
/// struct _TypeABC;
/// struct _TypeDEF;
///
/// impl GetBonuses<_TypeABC> for Test {
///     fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
///         None
///     }
/// }
///
/// impl GetBonuses<_TypeDEF> for Test {
///     fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
///         Some(Vec::new())
///     }
/// }
///
/// let abc_value = GetBonuses::<_TypeABC>::get_bonuses(&Test::A, 10.into());
/// let def_value = GetBonuses::<_TypeDEF>::get_bonuses(&Test::B, 10.into());
///
/// assert!(abc_value.is_none());
/// assert!(def_value.is_some());
/// ```
///
///
///
///
/// [`Attribute`]: crate::attribute::Attribute
/// [`Ability`]: crate::attribute::types::Ability
/// [`_AbilityScore`]: crate::attribute::types::_AbilityScore
/// [`_AbilityModifier`]: crate::attribute::types::_AbilityModifier
pub trait GetBonuses<T = ()> {
    /// Returns the bonuses for this object.
    ///
    /// `value` is the current value of this type.
    ///
    /// If there are no bonuses, for this object, this returns [`None`]. If there are bonuses, then
    /// a vector of each [`Bonus`] is returned.
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>>;
}
