use crate::{
    attribute::{types::Ability, Attribute, GetBonuses},
    bonus::{Bonus, BonusType},
};

use super::PlayerClass;

impl GetBonuses for PlayerClass {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        (value > 0f32).then(|| {
            let mut bonuses = vec![
                Bonus::new(
                    Attribute::CasterLevel((*self).into()),
                    BonusType::Stacking,
                    value.into(),
                    Attribute::from(*self).into(),
                    None,
                ),
                self.get_base_attack_bonus(value),
            ];

            if let Some(mut dc_bonuses) = self.get_ability_spell_dc_bonuses(value) {
                bonuses.append(&mut dc_bonuses);
            }

            bonuses
        })
    }
}

impl PlayerClass {
    fn get_base_attack_bonus(&self, value: f32) -> Bonus {
        let bab = match self {
            Self::Barbarian
            | Self::Fighter
            | Self::Paladin
            | Self::Ranger
            | Self::DarkHunter
            | Self::SacredFist => value,
            Self::Alchemist
            | Self::Artificer
            | Self::Bard
            | Self::Stormsinger
            | Self::Cleric
            | Self::DarkApostate
            | Self::Druid
            | Self::BlightCaster
            | Self::FavoredSoul
            | Self::Monk
            | Self::Rogue
            | Self::Warlock
            | Self::AcolyteOfTheSkin => (value * 0.75f32).floor(),
            Self::Sorcerer | Self::Wizard => (value * 0.5f32).floor(),
        };

        Bonus::new(
            Attribute::BaseAttackBonus,
            BonusType::Stacking,
            bab.into(),
            Attribute::from(*self).into(),
            None,
        )
    }

    fn get_ability_spell_dc_bonuses(&self, _: f32) -> Option<Vec<Bonus>> {
        match self {
            Self::Alchemist | Self::Artificer | Self::Wizard => Some(vec![Bonus::new(
                Attribute::SpellDC((*self).into()),
                BonusType::AbilityModifier,
                Attribute::AbilityModifier(Ability::Intelligence).into(),
                Attribute::from(*self).into(),
                None,
            )]),
            Self::Sorcerer
            | Self::Bard
            | Self::Stormsinger
            | Self::Warlock
            | Self::AcolyteOfTheSkin => Some(vec![Bonus::new(
                Attribute::SpellDC((*self).into()),
                BonusType::AbilityModifier,
                Attribute::AbilityModifier(Ability::Charisma).into(),
                Attribute::from(*self).into(),
                None,
            )]),
            Self::Cleric | Self::DarkApostate | Self::Druid | Self::BlightCaster => {
                Some(vec![Bonus::new(
                    Attribute::SpellDC((*self).into()),
                    BonusType::AbilityModifier,
                    Attribute::AbilityModifier(Ability::Wisdom).into(),
                    Attribute::from(*self).into(),
                    None,
                )])
            }
            Self::FavoredSoul => Some(vec![
                Bonus::new(
                    Attribute::SpellDC((*self).into()),
                    BonusType::AbilityModifier,
                    Attribute::AbilityModifier(Ability::Wisdom).into(),
                    Attribute::from(*self).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::SpellDC((*self).into()),
                    BonusType::AbilityModifier,
                    Attribute::AbilityModifier(Ability::Charisma).into(),
                    Attribute::from(*self).into(),
                    None,
                ),
            ]),
            _ => None,
        }
    }
}
