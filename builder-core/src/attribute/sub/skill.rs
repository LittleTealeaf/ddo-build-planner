use crate::{simple_enum, bonus::Bonus};

simple_enum!(Skill, (Balance "Balance", Bluff "Bluff", Concentration "Concentration", Diplomacy "Diplomacy", DisableDevice "DisableDevice", Haggle "Haggle", Heal "Heal", Hide "Hide", Intimidate "Intimidate", Jump "Jump", Listen "Listen", MoveSilently "Move Silently", OpenLock "Open Lock", Perform "Perform", Repair "Repair", Search "Search", SpellCraft "Spell Craft", Spot "Spot", Swim "Swim", Tumble "Tumble", UseMagicalDevice "Use Magical Device", All "All"));

impl Skill {
    pub fn get_cloned_skills(&self) -> Option<Vec<Skill>> {
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

    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            _ => None
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn cloned_skills_includes_all_skills() {
        let skills = Skill::All.get_cloned_skills().unwrap();

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
