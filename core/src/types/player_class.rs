use crate::{attribute::Attribute, traits::IterValues, types::level::PlayerLevel};

#[derive(
    Hash,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Debug,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
)]
pub enum PlayerClass {
    Alchemist,
    Artificer,
    Barbarian,
    Bard,
    Cleric,
    Druid,
    #[display("Favored Soul")]
    FavoredSoul,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
    #[display("Storm Singer")]
    StormSinger,
    #[display("Dark Apostate")]
    DarkApostate,
    #[display("Blight Caster")]
    BlightCaster,
    #[display("Dragon Disciple")]
    DragonDisciple,
    #[display("Sacred Fist")]
    SacredFist,
    #[display("Dark Hunter")]
    DarkHunter,
    #[display("Arcane Trickster")]
    ArcaneTrickster,
    #[display("Wild Mage")]
    WildMage,
    #[display("Acolyte of the Skin")]
    AcolyteOfTheSkin,
}

impl PlayerClass {
    pub const VALUES: [Self; 24] = [
        Self::Alchemist,
        Self::Artificer,
        Self::Barbarian,
        Self::Bard,
        Self::Cleric,
        Self::Druid,
        Self::FavoredSoul,
        Self::Fighter,
        Self::Monk,
        Self::Paladin,
        Self::Ranger,
        Self::Rogue,
        Self::Sorcerer,
        Self::Warlock,
        Self::Wizard,
        Self::StormSinger,
        Self::DarkApostate,
        Self::BlightCaster,
        Self::DragonDisciple,
        Self::SacredFist,
        Self::DarkHunter,
        Self::ArcaneTrickster,
        Self::WildMage,
        Self::AcolyteOfTheSkin,
    ];

    #[must_use]
    pub const fn level(self) -> Attribute {
        Attribute::Level(PlayerLevel::Heroic(self))
    }

    #[must_use]
    pub const fn hit_points_per_level(self) -> u32 {
        match self {
            Self::Barbarian => 12,
            Self::Ranger | Self::DarkHunter | Self::Fighter | Self::Paladin | Self::SacredFist => {
                10
            }
            Self::Monk
            | Self::DragonDisciple
            | Self::Rogue
            | Self::ArcaneTrickster
            | Self::Artificer
            | Self::Bard
            | Self::StormSinger
            | Self::Cleric
            | Self::Druid
            | Self::DarkApostate
            | Self::BlightCaster
            | Self::FavoredSoul => 8,
            Self::WildMage
            | Self::Sorcerer
            | Self::Wizard
            | Self::Warlock
            | Self::AcolyteOfTheSkin
            | Self::Alchemist => 6,
        }
    }
}

impl IterValues for PlayerClass {
    fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }
}
