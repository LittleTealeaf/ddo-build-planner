#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum PlayerClass {
    Barbarian,
    Bard,
    Stormsinger,
    Cleric,
    DarkApostate,
    Fighter,
    Paladin,
    SacredFist,
    Ranger,
    DarkHunter,
    Rogue,
    Sorcerer,
    Wizard,
    Monk,
    FavoredSoul,
    Artificer,
    Druid,
    Blightcaster,
    Warlock,
    AcolyteOfTheSkin,
    Alchemist,
}

impl ToString for PlayerClass {
    fn to_string(&self) -> String {
        String::from(match self {
            PlayerClass::Barbarian => "Barbarian",
            PlayerClass::Bard => "Bard",
            PlayerClass::Stormsinger => "Stormsinger",
            PlayerClass::Cleric => "Cleric",
            PlayerClass::DarkApostate => "Dark Apostate",
            PlayerClass::Fighter => "Fighter",
            PlayerClass::Paladin => "Paladin",
            PlayerClass::SacredFist => "Sacred Fist",
            PlayerClass::Ranger => "Ranger",
            PlayerClass::DarkHunter => "Dark Hunter",
            PlayerClass::Rogue => "Rogue",
            PlayerClass::Sorcerer => "Sorcerer",
            PlayerClass::Wizard => "Wizard",
            PlayerClass::Monk => "Monk",
            PlayerClass::FavoredSoul => "Favored Soul",
            PlayerClass::Artificer => "Artificer",
            PlayerClass::Druid => "Druid",
            PlayerClass::Blightcaster => "Blightcaster",
            PlayerClass::Warlock => "Warlock",
            PlayerClass::AcolyteOfTheSkin => "Acolyte of the Skin",
            PlayerClass::Alchemist => "Alchemist",
        })
    }
}
