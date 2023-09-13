use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusType},
    types::{Ability, PlayerClass},
};

impl GetBonuses for PlayerClass {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
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
    fn ability_bonus_to_spell_dc(self, ability: Ability) -> Bonus {
        Bonus::new(
            Attribute::SpellDC(self.into()),
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(ability).into(),
            Attribute::from(self).into(),
            None,
        )
    }

    fn get_ability_spell_dc_bonuses(self, _: f32) -> Option<Vec<Bonus>> {
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
