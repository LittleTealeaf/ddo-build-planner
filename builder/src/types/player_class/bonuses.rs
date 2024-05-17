use rust_decimal::Decimal;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{BonusTemplate, BonusType},
    types::{ability::Ability, player_class::PlayerClass},
};

impl GetBonuses for PlayerClass {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        // FIX: Check Logic for if zero-value bonuses are passed in
        let mut bonuses = vec![BonusTemplate::new(
            Attribute::CasterLevel((*self).into()),
            BonusType::Stacking,
            value,
            None,
        )];

        if let Some(mut dc_bonuses) = self.get_ability_spell_dc_bonuses(value) {
            bonuses.append(&mut dc_bonuses);
        }

        Some(bonuses)
    }
}

impl PlayerClass {
    fn ability_bonus_to_spell_dc(self, ability: Ability) -> BonusTemplate {
        BonusTemplate::new(
            Attribute::SpellDC(self.into()),
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(ability),
            None,
        )
    }

    fn get_ability_spell_dc_bonuses(self, _: Decimal) -> Option<Vec<BonusTemplate>> {
        match self {
            Self::Alchemist | Self::Artificer | Self::Wizard => {
                Some(vec![self.ability_bonus_to_spell_dc(Ability::Intelligence)])
            }
            Self::Sorcerer
            | Self::Bard
            | Self::Stormsinger
            | Self::Warlock
            | Self::AcolyteOfTheSkin => {
                Some(vec![self.ability_bonus_to_spell_dc(Ability::Charisma)])
            }
            Self::Cleric | Self::DarkApostate | Self::Druid | Self::BlightCaster => {
                Some(vec![self.ability_bonus_to_spell_dc(Ability::Wisdom)])
            }
            Self::FavoredSoul => Some(vec![
                self.ability_bonus_to_spell_dc(Ability::Wisdom),
                self.ability_bonus_to_spell_dc(Ability::Charisma),
            ]),
            _ => None,
        }
    }
}
