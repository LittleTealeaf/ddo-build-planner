use crate::{
    attribute::Attribute, bonus::Bonus,
    simple_enum,
};

pub const ALL_SKILLS: [Skill; 21] = [
    Skill::Balance,
    Skill::Bluff,
    Skill::Concentration,
    Skill::Diplomacy,
    Skill::DisableDevice,
    Skill::Haggle,
    Skill::Heal,
    Skill::Hide,
    Skill::Intimidate,
    Skill::Jump,
    Skill::Listen,
    Skill::MoveSilently,
    Skill::OpenLock,
    Skill::Perform,
    Skill::Repair,
    Skill::Search,
    Skill::SpellCraft,
    Skill::Spot,
    Skill::Swim,
    Skill::Tumble,
    Skill::UseMagicalDevice
];

simple_enum!(Skill, (Balance "Balance", Bluff "Bluff", Concentration "Concentration", Diplomacy "Diplomacy", DisableDevice "Disable Device", Haggle "Haggle", Heal "Heal", Hide "Hide", Intimidate "Intimidate", Jump "Jump", Listen "Listen", MoveSilently "Move Silently", OpenLock "Open Lock", Perform "Perform", Repair "Repair", Search "Search", SpellCraft "Spell Craft", Spot "Spot", Swim "Swim", Tumble "Tumble", UseMagicalDevice "Use Magical Device", All "All"));

macro_rules! skill_spell_power {
    ($skill: ident, $spellpower: ident, $value: expr) => {
        Bonus::new(
            $crate::attribute::Attribute::SpellPower(
                $crate::attribute::SpellPower::$spellpower,
            ),
            $crate::bonus::BonusType::Stacking,
            $value,
            $crate::bonus::BonusSource::Attribute(
                $crate::attribute::Attribute::Skill(Skill::$skill),
            ),
            None,
        )
    };
}

impl Skill {
    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Skill::Heal => Some(vec![
                skill_spell_power!(Heal, Positive, value),
                skill_spell_power!(Heal, Negative, value),
            ]),
            Skill::Perform => Some(vec![skill_spell_power!(Perform, Sonic, value)]),
            Skill::SpellCraft => Some(vec![
                skill_spell_power!(SpellCraft, Acid, value),
                skill_spell_power!(SpellCraft, Cold, value),
                skill_spell_power!(SpellCraft, Electric, value),
                skill_spell_power!(SpellCraft, Fire, value),
                skill_spell_power!(SpellCraft, Force, value),
                skill_spell_power!(SpellCraft, Light, value),
                skill_spell_power!(SpellCraft, Poison, value),
            ]),
            Skill::Repair => Some(vec![
                skill_spell_power!(Repair, Rust, value),
                skill_spell_power!(Repair, Repair, value),
            ]),
            _ => None,
        }
    }
}

impl From<Skill> for Attribute {
    fn from(value: Skill) -> Self {
        Attribute::Skill(value)
    }
}
