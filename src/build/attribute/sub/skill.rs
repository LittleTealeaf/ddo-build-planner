use crate::{
    attribute_subtype,
    build::{
        attribute::Attribute,
        bonus::{source::Source, types::BonusType, Bonus},
    },
};

use super::SpellDamageType;

attribute_subtype!(Skill, (Balance "Balance"), (Bluff "Bluff"), (Concentration "Concentration"), (Diplomacy "Diplomacy"), (DisableDevice "Disable Device"), (Haggle "Haggle"), (Heal "Heal"), (Hide "Hide"), (Intimidate "Intimidate"), (Jump "Jump"), (Listen "Listen"), (MoveSilently "Move Silently"), (OpenLock "Open Lock"), (Perform "Perform"), (Repair "Repair"), (Search "Search"), (SpellCraft "Spell Craft"), (Spot "Spot"), (Swim "Swim"), (Tumble "Tumble"), (UseMagicalDevice "Use Magical Device"));

impl Skill {
    pub fn get_bonuses(&self, value: f32, source: Source) -> Vec<Bonus> {
        match self {
            Skill::Heal => vec![
                Bonus::new(
                    Attribute::SpellPower(SpellDamageType::Positive),
                    BonusType::Skill,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPower(SpellDamageType::Negative),
                    BonusType::Skill,
                    value,
                    source,
                    None,
                ),
            ],
            Skill::SpellCraft => vec![
                Bonus::new(
                    Attribute::SpellPower(SpellDamageType::Acid),
                    BonusType::Skill,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPower(SpellDamageType::Cold),
                    BonusType::Skill,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPower(SpellDamageType::Electric),
                    BonusType::Skill,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPower(SpellDamageType::Fire),
                    BonusType::Skill,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPower(SpellDamageType::Force),
                    BonusType::Skill,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPower(SpellDamageType::Light),
                    BonusType::Skill,
                    value,
                    source,
                    None,
                ),
            ],
            Skill::Perform => vec![Bonus::new(
                Attribute::SpellPower(SpellDamageType::Sonic),
                BonusType::Skill,
                value,
                source,
                None,
            )],
            _ => Vec::new(),
        }
    }
}
