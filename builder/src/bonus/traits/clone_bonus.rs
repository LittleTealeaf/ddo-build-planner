use crate::bonus::Bonus;

/// Provides the function [`clone_bonus()`], which calculates clones of a given bonus.
///
/// This trait is intended to be implemented by [`Attribute`] and subtypes of [`Attribute`]. Then,
/// it is called by [`Compiler`] to automatically populate any cloned bonus.
///
/// One use-case for this trait is for situations where an `All` option is present (such as
/// [`Skill::All`]) that needs to be split off into each individual value.
///
/// If there are no cloned bonuses, [`None`] is returned.
///
/// [`Skill::All`]: crate::attribute::types::Skill::All
/// [`Compiler`]: crate::compiler::Compiler
/// [`Attribute`]: crate::attribute::Attribute
/// [`clone_bonus()`]: crate::bonus::CloneBonus::clone_bonus
pub trait CloneBonus {
    /// Clones the provided bonus if there are any cloned variations.
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>>;
}

/// Provides a standard implementation of [`CloneBonus`] for instances where
/// a specific enum value corresponds to a static public const array in the data type.
///
/// For example
///
#[macro_export]
macro_rules! implement_clone_bonus_array {
    ($datatype: ident, $all: ident, $array: ident) => {
        impl CloneBonus for $datatype {
            fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
                match self {
                    Self::$all => Some(
                        Self::$array
                            .map(|ability| bonus.clone_into_attribute(ability))
                            .to_vec(),
                    ),
                    _ => None,
                }
            }
        }
    };
}
