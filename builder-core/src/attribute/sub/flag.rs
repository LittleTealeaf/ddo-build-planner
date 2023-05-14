use crate::{bonus::Bonus, simple_enum};

use super::{Ability, SavingThrow, Toggle};

simple_enum!(
    Flag, (
        Centered() String::from("Centered"),
        Toggle(toggle: Toggle) format!("Toggled: {}", toggle.to_string()),
        AbilityToSavingThrow(ability: Ability, savingthrow: SavingThrow) format!("{} to {} saving throw", ability.to_string(), savingthrow.to_string()),
        ReligiousLoreToQualityMagicalSheltering() String::from("Religious Lore to Quality Magical Sheltering"),
        ReligiousLoreToQualityPhysicalSheltering() String::from("Religious Lore to Quality Physical Sheltering")
    )
);

impl Flag {
    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Self::Toggle(toggle) => toggle.get_toggled_bonuses(value),
            _ => None,
        }
    }
}
