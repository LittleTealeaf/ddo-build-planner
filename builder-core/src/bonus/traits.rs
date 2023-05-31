use super::Bonus;

/// Provides the function [`clone_bonus()`], which calculates clones of a given bonus.
///
/// This trait is intended to be implemented by [`Attribute`] and sub-types of [`Attribute`]. Then,
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
