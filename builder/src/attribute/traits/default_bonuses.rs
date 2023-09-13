
// TODO: more detailed documentation

use crate::bonus::Bonus;

/// Implements the ability to have default bonuses for a trait.
///
/// Default bonuses are implemented on initialization of [`Compilers`]
///
/// [`Compilers`]: crate::compiler::Compiler
pub trait DefaultBonuses {
    /// The type of iterable that will be returned by the function.
    /// As default bonuses are static, the prefered return type is some form of array
    type Iterator: IntoIterator<Item = Bonus>;

    /// Returns the default bonuses, if there are any
    fn get_default_bonuses() -> Self::Iterator;
}