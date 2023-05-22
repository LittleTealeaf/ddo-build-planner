use crate::{
    attribute::{Attribute, Flag, GetBonuses},
    bonus::{Bonus, BonusType, Condition},
    simple_enum,
};

simple_enum!(ClassLore, "", (Religious "Religious", Arcane "Arcane", Wilderness "Wilderness"));

impl GetBonuses for ClassLore {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Self::Religious => Some(vec![
                Bonus::new(
                    Attribute::MagicalSheltering(),
                    BonusType::Quality,
                    value,
                    Attribute::to_source(ClassLore::Religious),
                    Some(vec![Condition::Has(
                        Flag::ReligiousLoreToQualityMagicalSheltering().into(),
                    )]),
                ),
                Bonus::new(
                    Attribute::PhysicalSheltering(),
                    BonusType::Quality,
                    value,
                    Attribute::to_source(ClassLore::Religious),
                    Some(vec![Condition::Has(
                        Flag::ReligiousLoreToQualityPhysicalSheltering().into(),
                    )]),
                ),
            ]),
            _ => None,
        }
    }
}

impl From<ClassLore> for Attribute {
    #[inline(always)]
    fn from(value: ClassLore) -> Self {
        Attribute::ClassLore(value)
    }
}
