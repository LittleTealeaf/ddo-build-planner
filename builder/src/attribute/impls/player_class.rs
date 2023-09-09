use rust_decimal::Decimal;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusType},
    types::{Ability, PlayerClass},
};

impl GetBonuses for PlayerClass {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<crate::bonus::Bonus>> {
        let mut bonuses = vec![Bonus::new(
            Attribute::CasterLevel((*self).into()),
            BonusType::Stacking,
            value.into(),
            Attribute::from(*self).into(),
            None,
        )];

        if let Some(mut dc_bonuses) = self.get_ability_spell_dc_bonuses(value) {
            bonuses.append(&mut dc_bonuses);
        }

        Some(bonuses)
    }
}

impl PlayerClass {
    fn get_ability_spell_dc_bonuses(self, _: Decimal) -> Option<Vec<Bonus>> {
        match self {
            Self::Alchemist | Self::Artificer | Self::Wizard => Some(vec![Bonus::new(
                Attribute::SpellDC(self.into()),
                BonusType::AbilityModifier,
                Attribute::AbilityModifier(Ability::Intelligence).into(),
                Attribute::from(self).into(),
                None,
            )]),
            Self::Sorcerer
            | Self::Bard
            | Self::Stormsinger
            | Self::Warlock
            | Self::AcolyteOfTheSkin => Some(vec![Bonus::new(
                Attribute::SpellDC(self.into()),
                BonusType::AbilityModifier,
                Attribute::AbilityModifier(Ability::Charisma).into(),
                Attribute::from(self).into(),
                None,
            )]),
            Self::Cleric | Self::DarkApostate | Self::Druid | Self::BlightCaster => {
                Some(vec![Bonus::new(
                    Attribute::SpellDC(self.into()),
                    BonusType::AbilityModifier,
                    Attribute::AbilityModifier(Ability::Wisdom).into(),
                    Attribute::from(self).into(),
                    None,
                )])
            }
            Self::FavoredSoul => Some(vec![
                Bonus::new(
                    Attribute::SpellDC(self.into()),
                    BonusType::AbilityModifier,
                    Attribute::AbilityModifier(Ability::Wisdom).into(),
                    Attribute::from(self).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::SpellDC(self.into()),
                    BonusType::AbilityModifier,
                    Attribute::AbilityModifier(Ability::Charisma).into(),
                    Attribute::from(self).into(),
                    None,
                ),
            ]),
            _ => None,
        }
    }
}
