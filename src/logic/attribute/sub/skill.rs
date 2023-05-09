use crate::logic::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType},
};

use super::SpellPower;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Skill {
    Balance,
    Bluff,
    Concentration,
    Diplomacy,
    DisableDevice,
    Haggle,
    Heal,
    Hide,
    Intimidate,
    Jump,
    Listen,
    MoveSilently,
    OpenLock,
    Perform,
    Repair,
    Search,
    SpellCraft,
    Spot,
    Swim,
    Tumble,
    UseMagicalDevice,
}

impl Skill {
    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Skill::Heal => Some(vec![
                Bonus::new(
                    Attribute::SpellPower(SpellPower::Positive),
                    BonusType::Stacking,
                    value,
                    BonusSource::Attribute(Attribute::Skill(Skill::Heal)),
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPower(SpellPower::Negative),
                    BonusType::Stacking,
                    value,
                    BonusSource::Attribute(Attribute::Skill(Skill::Heal)),
                    None,
                ),
            ]),
            Skill::Perform => Some(vec![Bonus::new(
                Attribute::SpellPower(SpellPower::Sonic),
                BonusType::Stacking,
                value,
                BonusSource::Attribute(Attribute::Skill(Skill::Heal)),
                None,
            )]),
            Skill::SpellCraft => Some(vec![
                Bonus::new(
                    Attribute::SpellPower(SpellPower::Acid),
                    BonusType::Stacking,
                    value,
                    BonusSource::Attribute(Attribute::Skill(Skill::SpellCraft)),
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPower(SpellPower::Cold),
                    BonusType::Stacking,
                    value,
                    BonusSource::Attribute(Attribute::Skill(Skill::SpellCraft)),
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPower(SpellPower::Electric),
                    BonusType::Stacking,
                    value,
                    BonusSource::Attribute(Attribute::Skill(Skill::SpellCraft)),
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPower(SpellPower::Fire),
                    BonusType::Stacking,
                    value,
                    BonusSource::Attribute(Attribute::Skill(Skill::SpellCraft)),
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPower(SpellPower::Force),
                    BonusType::Stacking,
                    value,
                    BonusSource::Attribute(Attribute::Skill(Skill::SpellCraft)),
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPower(SpellPower::Light),
                    BonusType::Stacking,
                    value,
                    BonusSource::Attribute(Attribute::Skill(Skill::SpellCraft)),
                    None,
                ),
                // TODO: Figure out which skill improves poison spell power
            ]),
            Skill::Repair => Some(vec![
                Bonus::new(
                    Attribute::SpellPower(SpellPower::Rust),
                    BonusType::Stacking,
                    value,
                    BonusSource::Attribute(Attribute::Skill(Skill::Repair)),
                    None,
                ),
                Bonus::new(
                    Attribute::SpellPower(SpellPower::Repair),
                    BonusType::Stacking,
                    value,
                    BonusSource::Attribute(Attribute::Skill(Skill::Repair)),
                    None,
                ),
            ]),
            _ => None,
        }
    }
}

impl ToString for Skill {
    fn to_string(&self) -> String {
        String::from(match self {
            Skill::Balance => "Balance",
            Skill::Bluff => "Bluff",
            Skill::Concentration => "Concentration",
            Skill::Diplomacy => "Diplomacy",
            Skill::DisableDevice => "Disable Device",
            Skill::Haggle => "Haggle",
            Skill::Heal => "Heal",
            Skill::Hide => "Hide",
            Skill::Intimidate => "Intimidate",
            Skill::Jump => "Jump",
            Skill::Listen => "Listen",
            Skill::MoveSilently => "Move Silently",
            Skill::OpenLock => "Open Lock",
            Skill::Perform => "Perform",
            Skill::Repair => "Repair",
            Skill::Search => "Search",
            Skill::SpellCraft => "Spell Craft",
            Skill::Spot => "Spot",
            Skill::Swim => "Swim",
            Skill::Tumble => "Tumble",
            Skill::UseMagicalDevice => "Use Magical Device",
        })
    }
}
