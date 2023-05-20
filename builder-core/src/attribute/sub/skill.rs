use crate::{bonus::Bonus, simple_enum, attribute::{GetCloned, Attribute}};

simple_enum!(Skill, "", (Balance "Balance", Bluff "Bluff", Concentration "Concentration", Diplomacy "Diplomacy", DisableDevice "DisableDevice", Haggle "Haggle", Heal "Heal", Hide "Hide", Intimidate "Intimidate", Jump "Jump", Listen "Listen", MoveSilently "Move Silently", OpenLock "Open Lock", Perform "Perform", Repair "Repair", Search "Search", SpellCraft "Spell Craft", Spot "Spot", Swim "Swim", Tumble "Tumble", UseMagicalDevice "Use Magical Device", All "All"));

macro_rules! spell_power {
    ($skill: ident, $spell_power: ident, $value: expr) => {
        $crate::bonus::Bonus::new(
            $crate::attribute::Attribute::SpellPower($crate::attribute::SpellPower::$spell_power),
            $crate::bonus::BonusType::Stacking,
            $value,
            $crate::bonus::BonusSource::Attribute($crate::attribute::Attribute::Skill(
                Skill::$skill,
            )),
            None,
        )
    };
}

impl Skill {
    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Skill::Heal => Some(vec![
                spell_power!(Heal, Positive, value),
                spell_power!(Heal, Negative, value),
            ]),
            Skill::Perform => Some(vec![spell_power!(Perform, Sonic, value)]),
            Skill::SpellCraft => Some(vec![
                spell_power!(SpellCraft, Acid, value),
                spell_power!(SpellCraft, Cold, value),
                spell_power!(SpellCraft, Electric, value),
                spell_power!(SpellCraft, Fire, value),
                spell_power!(SpellCraft, Force, value),
                spell_power!(SpellCraft, Light, value),
                spell_power!(SpellCraft, Poison, value),
            ]),
            _ => None,
        }
    }
}

impl GetCloned<Skill> for Skill {
    fn get_cloned(&self) -> Option<Vec<Skill>> {
        match self {
            Self::All => Some(vec![
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
                Skill::UseMagicalDevice,
            ]),
            _ => None,
        }
    }
}

impl From<Skill> for Attribute {
    fn from(value: Skill) -> Attribute {
        Attribute::Skill(value)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn cloned_skills_includes_all_skills() {
        let skills = Skill::All.get_cloned().unwrap();

        for skill in [
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
            Skill::UseMagicalDevice,
        ] {
            assert!(skills.contains(&skill));
        }
    }
}
