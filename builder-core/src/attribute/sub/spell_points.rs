use crate::{attribute::Attribute, simple_enum};

simple_enum!(SpellPoint, "", (Base "Base Spell Points", BaseScalar "Base Spell Point Scalar", Bonus "Bonus Spell Points", Scalar "Spell Points Scalar"));

impl From<SpellPoint> for Attribute {
    #[inline(always)]
    fn from(value: SpellPoint) -> Self {
        Attribute::SpellPoints(value)
    }
}
