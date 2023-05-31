use crate::attribute::Attribute;

/// Represents a value of a [`Bonus`]
///
/// [`Bonus`]: crate::bonus::Bonus
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BonusValue {
    /// Just a simple [`f32`] value.
    Value(f32),
    /// Copy the total value of some [`Attribute`].
    FromAttribute(Attribute),
    /// Scale some total value of some [`Attribute`] by some value.
    ScaleAttribute(Attribute, f32),
}

impl BonusValue {
    /// Returns any dependencies associated with the value.
    ///
    /// In short terms: If the [`BonusValue`] has an [`Attribute`] in it, then this returns a
    /// [`Vec`] with all attributes included.
    pub fn get_dependencies(&self) -> Option<Vec<Attribute>> {
        match self {
            Self::ScaleAttribute(attr, _) | Self::FromAttribute(attr) => Some(vec![*attr]),
            _ => None,
        }
    }
}

impl From<f32> for BonusValue {
    fn from(value: f32) -> BonusValue {
        BonusValue::Value(value)
    }
}

impl From<Attribute> for BonusValue {
    fn from(value: Attribute) -> Self {
        BonusValue::FromAttribute(value)
    }
}

impl From<(Attribute, f32)> for BonusValue {
    fn from((attribute, scale): (Attribute, f32)) -> Self {
        BonusValue::ScaleAttribute(attribute, scale)
    }
}
