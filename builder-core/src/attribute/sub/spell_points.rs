use crate::{simple_enum, attribute::Attribute};

simple_enum!(SpellPoint, "", (Base "Base Spell Points", BaseScalar "Base Spell Point Scalar", Bonus "Bonus Spell Points", Scalar "Spell Points Scalar"));

impl From<SpellPoint> for Attribute {
    fn from(value: SpellPoint) -> Self {
        Attribute::SpellPoints(value)
    }
}
