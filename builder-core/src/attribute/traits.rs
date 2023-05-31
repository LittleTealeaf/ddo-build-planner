use crate::bonus::Bonus;

/// Implements the ability to get bonuses from different [`Attribute`] sub-types.
///
/// The generic type `T` is used as a means of differentiating different bonuses for an
/// [`Attribute`] sub-type that has multiple instances. `T` is indeded to be a 0-size struct. An
/// example of this is with [`Ability`], which has the two helper structs [`_AbilityScore`] and
/// [`_AbilityModifier`].
///
/// # Examples
///
/// ```
/// use builder_core::{bonus::Bonus, attribute::GetBonuses};
///
/// enum Test {
///     A,
///     B,
/// }
///
/// impl GetBonuses for Test {
///     fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
///         // implementation
///         None // return None if there are none
///     }
/// }
///
/// let value = Test::A.get_bonuses(10f32);
/// assert!(value.is_none());
///
/// ```
///
/// If there are no bonuses for a type, [`None`] is returned.
///
/// ```
/// use builder_core::{bonus::Bonus, attribute::GetBonuses};
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
///     fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
///         None
///     }
/// }
///
/// impl GetBonuses<_TypeDEF> for Test {
///     fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
///         Some(Vec::new())
///     }
/// }
///
/// let abc_value = GetBonuses::<_TypeABC>::get_bonuses(&Test::A, 10f32);
/// let def_value = GetBonuses::<_TypeDEF>::get_bonuses(&Test::B, 10f32);
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
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>>;
}


/// Implements the check of whether a particular attribute is actually tracked. 
///
/// Most likely, this
/// will be `true`, but there are some bonuses that should not be tracked and therefore should be
/// ignored.
pub trait TrackAttribute {
    /// Checks whether or not the object should be tracked.
    ///
    /// If the object should be tracked, returns `true`, otherwise returns `false`
    fn is_tracked(&self) -> bool;
}
