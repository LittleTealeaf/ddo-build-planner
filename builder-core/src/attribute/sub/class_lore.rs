use crate::{
    attribute::{Attribute, Flag},
    bonus::{Bonus, BonusSource, BonusType, Condition},
    simple_enum,
};

simple_enum!(ClassLore, "", (Religious "Religious", Arcane "Arcane", Wilderness "Wilderness"));

impl ClassLore {
    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Self::Religious => Some(vec![
                Bonus::new(
                    Attribute::MagicalSheltering(),
                    BonusType::Quality,
                    value,
                    BonusSource::Attribute(Attribute::ClassLore(ClassLore::Religious)),
                    Some(vec![Condition::Has(Attribute::Flag(
                        Flag::ReligiousLoreToQualityMagicalSheltering(),
                    ))]),
                ),
                Bonus::new(
                    Attribute::PhysicalSheltering(),
                    BonusType::Quality,
                    value,
                    BonusSource::Attribute(Attribute::ClassLore(ClassLore::Religious)),
                    Some(vec![Condition::Has(Attribute::Flag(
                        Flag::ReligiousLoreToQualityPhysicalSheltering(),
                    ))]),
                ),
            ]),
            _ => None,
        }
    }
}

impl From<ClassLore> for Attribute {
    fn from(value: ClassLore) -> Self {
        Attribute::ClassLore(value)
    }
}
