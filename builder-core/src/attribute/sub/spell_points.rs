use crate::{
    attribute::Attribute, breakdown::Breakdowns,
    simple_enum,
};

simple_enum!(SpellPoint, (Points "Spell Points", BonusPoints "Bonus Spell Points", Scalar "Spell Points Scalar", BonusScalar "Bonus Spell Point Scalar"));

impl Breakdowns {
    pub fn get_total_spell_points(&mut self) -> f32 {
        (self.get_attribute(&SpellPoint::Points.into())
            + self.get_attribute(&SpellPoint::BonusPoints.into())
                * (1f32 + self.get_attribute(&SpellPoint::BonusScalar.into())))
            * (1f32 + self.get_attribute(&SpellPoint::Scalar.into()))
    }
}

impl From<SpellPoint> for Attribute {
    fn from(value: SpellPoint) -> Self {
        Attribute::SpellPoints(value)
    }
}
