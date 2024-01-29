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

/// Calculates the depth of the object. Depth refers to the number of objects nested within this
/// instance.
///
/// For example, A max of a max has a depth of 2, because it has nested values.
pub trait Depth {
    /// Returns the maximum depth of this object
    fn get_depth(&self) -> usize;
}

/// Declares whether or not this object contains dice in it's value
pub trait HasDice {
    /// returns `true` if this object has some [`Value::Dice`] within its calculation
    fn has_dice(&self) -> bool;
}
