use super::{Attributable, Attribute};

#[derive(Clone, Copy)]
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

impl Attributable for Skill {
    fn into_attribute(self) -> super::Attribute {
        Attribute::Skill(self)
    }
}
