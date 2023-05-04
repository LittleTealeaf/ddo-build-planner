#[derive(Clone, PartialEq, Eq, Hash, Copy)]
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
    Spellcraft,
    Spot,
    Swim,
    Tumble,
    UseMagicalDevice,
}

impl ToString for Skill {
    fn to_string(&self) -> String {
        String::from(match self {
            Skill::Bluff => "Bluff",
            Skill::Concentration => "Concentration",
            Skill::Diplomacy => "Diplomacy",
            Skill::Haggle => "Haggle",
            Skill::Heal => "Heal",
            Skill::Intimidate => "Intimidate",
            Skill::Jump => "Jump",
            Skill::Listen => "Listen",
            Skill::Search => "Search",
            Skill::Spellcraft => "Spellcraft",
            Skill::Swim => "Swim",
            Skill::Tumble => "Tumble",
            Skill::UseMagicalDevice => "Use Magical Device",
            Skill::Balance => "Balance",
            Skill::DisableDevice => "Disable Device",
            Skill::Hide => "Hide",
            Skill::MoveSilently => "Move Silently",
            Skill::OpenLock => "Open Lock",
            Skill::Perform => "Perform",
            Skill::Repair => "Repair",
            Skill::Spot => "Spot",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Skill;

    #[test]
    fn has_to_string() {
        [
            Skill::Bluff,
            Skill::Concentration,
            Skill::Diplomacy,
            Skill::Haggle,
            Skill::Heal,
            Skill::Intimidate,
            Skill::Jump,
            Skill::Listen,
            Skill::Search,
            Skill::Spellcraft,
            Skill::Swim,
            Skill::Tumble,
            Skill::UseMagicalDevice,
            Skill::Balance,
            Skill::DisableDevice,
            Skill::Hide,
            Skill::MoveSilently,
            Skill::OpenLock,
            Skill::Perform,
            Skill::Repair,
            Skill::Spot,
        ]
        .map(|skill| {
            let string = skill.to_string();
            assert!(string.len() > 0);
        });
    }
}
