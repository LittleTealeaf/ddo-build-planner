use crate::simple_enum;

mod class_bonuses;
pub use class_bonuses::*;

simple_enum!(PlayerClass, "", (
    Alchemist "Alchemist",
    Artificer "Artificer",
    Barbarian "Barbarian",
    Fighter "Fighter",
    Monk "Monk",
    Rogue "Rogue",
    Sorcerer "Sorcerer",
    Wizard "Wiazard",
    FavoredSoul "Favored Soul",
    Bard "Bard",
    Stormsinger "Stormsinger",
    Cleric "Cleric",
    DarkApostate "Dark Apostate",
    Druid "Druid",
    Blightcaster "Blightcaster",
    Paladin "Paladin",
    SacredFist "SacredFist",
    Ranger "Ranger",
    DarkHunter "DarkHunter",
    Warlock "Warlock",
    AcolyteOfTheSkin "AcolyteOfTheSkin"
));
