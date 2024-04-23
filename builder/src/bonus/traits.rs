mod clone_bonus;
pub use clone_bonus::*;

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
