use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utils::public_modules;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusSource, BonusType, Value},
    feat::{Feat, FeatRequirement, GetFeatRequirement},
    types::{Ability, PlayerClass, Skill, SpellPoints, SpellPower},
};

public_modules!(spell_focus);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
/// Feats thga fall under the "Spellcasting" category
pub enum SpellcastingFeat {
    /// Feats that provide bonuses to spell DCs
    SpellFocus(SpellFocusFeat),
    /// Magical Training Feat
    MagicalTraining,
    /// Mental Toughness Feat
    MentalToughness,
    /// Improved Mental Toughness Feat
    ImprovedMentalToughness,
    /// Spell Penetration
    SpellPenetration,
    /// Improved Spell Penetration
    GreaterSpellPenetration,
    /// Combat Casting
    CombatCasting,
    /// Mobile Spellcasting
    MobileSpellcasting,
    /// Augment Summoning
    AugmentSummoning,
}

impl GetBonuses for SpellcastingFeat {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        (value > 0f32).then(|| match self {
            Self::AugmentSummoning => Some(vec![Bonus::new(
                Attribute::SummonedAttribute(crate::types::SummonedAttribute::AbilityScore(
                    Ability::All,
                )),
                BonusType::Stacking,
                4f32.into(),
                BonusSource::Attribute(Attribute::Feat(Feat::Spellcasting(*self))),
                None,
            )]),
            Self::MobileSpellcasting => None,
            Self::SpellFocus(focus) => focus.get_bonuses(value),
            Self::CombatCasting => Some(vec![Bonus::new(
                Attribute::Skill(Skill::Concentration),
                BonusType::Stacking,
                4f32.into(),
                BonusSource::Attribute(Attribute::Feat(Feat::Spellcasting(*self))),
                None,
            )]),
            Self::MagicalTraining => Some(vec![
                Bonus::new(
                    Attribute::SpellCriticalChance(SpellPower::Potency),
                    BonusType::Stacking,
                    5f32.into(),
                    BonusSource::Attribute(Attribute::Feat(Feat::Spellcasting(*self))),
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPoints(SpellPoints::Base),
                    BonusType::Stacking,
                    Value::Value(80f32),
                    BonusSource::Attribute(Attribute::Feat(Feat::Spellcasting(*self))),
                    None,
                ),
            ]),
            Self::MentalToughness | Self::ImprovedMentalToughness => Some(vec![
                Bonus::new(
                    Attribute::SpellCriticalChance(SpellPower::Potency),
                    BonusType::Stacking,
                    1f32.into(),
                    BonusSource::Attribute(Attribute::Feat(Feat::Spellcasting(*self))),
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPoints(SpellPoints::Base),
                    BonusType::Stacking,
                    Value::Sum(vec![
                        Value::Value(5f32),
                        Value::Product(vec![
                            Value::Attribute(Attribute::TotalCharacterLevel),
                            Value::Value(5f32),
                        ]),
                    ]),
                    BonusSource::Attribute(Attribute::Feat(Feat::Spellcasting(*self))),
                    None,
                ),
            ]),
            Self::SpellPenetration | Self::GreaterSpellPenetration => Some(vec![Bonus::new(
                Attribute::SpellPenetration,
                BonusType::Stacking,
                2f32.into(),
                BonusSource::Attribute(Attribute::Feat(Feat::Spellcasting(*self))),
                None,
            )]),
        })?
    }
}

impl GetFeatRequirement for SpellcastingFeat {
    fn get_feat_requirements(&self) -> Option<FeatRequirement> {
        match self {
            Self::AugmentSummoning | Self::MagicalTraining => None,
            Self::SpellFocus(focus) => focus.get_feat_requirements(),
            Self::MentalToughness | Self::SpellPenetration | Self::CombatCasting => {
                Some(FeatRequirement::Any(vec![
                    FeatRequirement::ClassLevel(PlayerClass::Cleric, 1),
                    FeatRequirement::ClassLevel(PlayerClass::Bard, 1),
                    FeatRequirement::ClassLevel(PlayerClass::Sorcerer, 1),
                    FeatRequirement::ClassLevel(PlayerClass::Wizard, 1),
                    FeatRequirement::ClassLevel(PlayerClass::Ranger, 4),
                    FeatRequirement::ClassLevel(PlayerClass::DarkHunter, 4),
                    FeatRequirement::ClassLevel(PlayerClass::Paladin, 4),
                    FeatRequirement::ClassLevel(PlayerClass::FavoredSoul, 1),
                    FeatRequirement::ClassLevel(PlayerClass::Artificer, 1),
                    FeatRequirement::ClassLevel(PlayerClass::Druid, 1),
                    FeatRequirement::ClassLevel(PlayerClass::Warlock, 1),
                    FeatRequirement::ClassLevel(PlayerClass::Alchemist, 1),
                    FeatRequirement::ClassLevel(PlayerClass::SacredFist, 4),
                    FeatRequirement::ClassLevel(PlayerClass::BlightCaster, 1),
                    FeatRequirement::ClassLevel(PlayerClass::DarkApostate, 1),
                    FeatRequirement::ClassLevel(PlayerClass::Stormsinger, 1),
                    FeatRequirement::ClassLevel(PlayerClass::AcolyteOfTheSkin, 1),
                ]))
            }
            Self::GreaterSpellPenetration => Some(FeatRequirement::Feat(Feat::Spellcasting(
                Self::SpellPenetration,
            ))),
            Self::ImprovedMentalToughness => Some(FeatRequirement::Any(vec![
                FeatRequirement::ClassLevel(PlayerClass::Cleric, 5),
                FeatRequirement::ClassLevel(PlayerClass::Bard, 7),
                FeatRequirement::ClassLevel(PlayerClass::Sorcerer, 6),
                FeatRequirement::ClassLevel(PlayerClass::Wizard, 5),
                FeatRequirement::ClassLevel(PlayerClass::Ranger, 10),
                FeatRequirement::ClassLevel(PlayerClass::DarkHunter, 10),
                FeatRequirement::ClassLevel(PlayerClass::Paladin, 10),
                FeatRequirement::ClassLevel(PlayerClass::FavoredSoul, 6),
                FeatRequirement::ClassLevel(PlayerClass::Artificer, 7),
                FeatRequirement::ClassLevel(PlayerClass::Druid, 5),
                FeatRequirement::ClassLevel(PlayerClass::Warlock, 7),
                FeatRequirement::ClassLevel(PlayerClass::Alchemist, 7),
                FeatRequirement::ClassLevel(PlayerClass::SacredFist, 10),
                FeatRequirement::ClassLevel(PlayerClass::BlightCaster, 5),
                FeatRequirement::ClassLevel(PlayerClass::DarkApostate, 5),
                FeatRequirement::ClassLevel(PlayerClass::Stormsinger, 7),
                FeatRequirement::ClassLevel(PlayerClass::AcolyteOfTheSkin, 7),
            ])),
            Self::MobileSpellcasting => Some(FeatRequirement::All(vec![
                FeatRequirement::Feat(Feat::Spellcasting(Self::CombatCasting)),
                FeatRequirement::AbilityScore(Ability::Dexterity, 13),
                FeatRequirement::Any(vec![
                    FeatRequirement::ClassLevel(PlayerClass::Cleric, 3),
                    FeatRequirement::ClassLevel(PlayerClass::Bard, 4),
                    FeatRequirement::ClassLevel(PlayerClass::Sorcerer, 4),
                    FeatRequirement::ClassLevel(PlayerClass::Wizard, 3),
                    FeatRequirement::ClassLevel(PlayerClass::Ranger, 8),
                    FeatRequirement::ClassLevel(PlayerClass::Paladin, 8),
                    FeatRequirement::ClassLevel(PlayerClass::FavoredSoul, 4),
                    FeatRequirement::ClassLevel(PlayerClass::Artificer, 4),
                    FeatRequirement::ClassLevel(PlayerClass::Druid, 1),
                    FeatRequirement::ClassLevel(PlayerClass::Warlock, 4),
                    FeatRequirement::ClassLevel(PlayerClass::Alchemist, 4),
                    FeatRequirement::ClassLevel(PlayerClass::SacredFist, 8),
                    FeatRequirement::ClassLevel(PlayerClass::BlightCaster, 1),
                    FeatRequirement::ClassLevel(PlayerClass::DarkApostate, 3),
                    FeatRequirement::ClassLevel(PlayerClass::Stormsinger, 4),
                    FeatRequirement::ClassLevel(PlayerClass::AcolyteOfTheSkin, 4),
                    FeatRequirement::ClassLevel(PlayerClass::DarkHunter, 8),
                ]),
            ])),
        }
    }
}

impl Display for SpellcastingFeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AugmentSummoning => write!(f, "Augment Summoning"),
            Self::SpellFocus(feat) => feat.fmt(f),
            Self::MentalToughness => write!(f, "Mental Toughness"),
            Self::ImprovedMentalToughness => write!(f, "Improved Mental Toughness"),
            Self::MagicalTraining => write!(f, "Magical Training"),
            Self::SpellPenetration => write!(f, "Spell Penetration"),
            Self::GreaterSpellPenetration => write!(f, "Greater Spell Penetration"),
            Self::CombatCasting => write!(f, "Combat Casting"),
            Self::MobileSpellcasting => write!(f, "Mobile Spellcasting"),
        }
    }
}
