use crate::{
    logic::{attribute::Attribute, breakdown::Breakdowns},
    simple_attribute_enum,
};

simple_attribute_enum!(SpellPoints, (SpellPoints "Spell Points", BonusSpellPoints "Bonus Spell Points", SpellPointScalar "Spell Points Scalar", BonusSpellPointScalar "Bonus Spell Point Scalar"));

impl Breakdowns {
    pub fn get_total_spell_points(&mut self) -> f32 {
        (self.get_attribute(&SpellPoints::SpellPoints.into())
            + self.get_attribute(&SpellPoints::BonusSpellPoints.into())
                * (1f32 + self.get_attribute(&SpellPoints::BonusSpellPointScalar.into())))
            * (1f32 + self.get_attribute(&SpellPoints::SpellPointScalar.into()))
    }
}

impl From<SpellPoints> for Attribute {
    fn from(value: SpellPoints) -> Self {
        Attribute::SpellPoints(value)
    }
}
