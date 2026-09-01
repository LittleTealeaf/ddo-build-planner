use itertools::chain;

use crate::{
    bonus::{
        traits::{ToAttribute, ToValue},
        Bonus, BonusType,
    },
    types::player_class::PlayerClass,
    val,
};

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
    strum::VariantArray,
)]
pub enum Lore {
    #[display("Religious Lore")]
    Religious,
    #[display("Arcane Lore")]
    Arcane,
    #[display("Wilderness Lore")]
    Wilderness,
}

impl Lore {
    pub fn core_bonuses() -> impl Iterator<Item = Bonus> {
        chain!(
            [
                (PlayerClass::Cleric, Self::Religious),
                (PlayerClass::DarkApostate, Self::Religious),
                (PlayerClass::FavoredSoul, Self::Religious),
                (PlayerClass::Paladin, Self::Religious),
                (PlayerClass::SacredFist, Self::Religious),
                (PlayerClass::Artificer, Self::Arcane),
                (PlayerClass::Alchemist, Self::Arcane),
                (PlayerClass::Bard, Self::Arcane),
                (PlayerClass::Sorcerer, Self::Arcane),
                (PlayerClass::StormSinger, Self::Arcane),
                (PlayerClass::WildMage, Self::Arcane),
                (PlayerClass::Wizard, Self::Arcane),
                (PlayerClass::Barbarian, Self::Wilderness),
                (PlayerClass::BlightCaster, Self::Wilderness),
                (PlayerClass::DarkHunter, Self::Wilderness),
                (PlayerClass::Druid, Self::Wilderness),
                (PlayerClass::Ranger, Self::Wilderness)
            ]
            .map(|(cls, lore)| {
                Bonus::new(
                    lore.attribute(),
                    cls.level().to_value(),
                    BonusType::Stacking,
                    cls.level(),
                )
                .with_show_condition(cls.level().to_value().is_some())
            }),
            [PlayerClass::Bard, PlayerClass::StormSinger]
                .into_iter()
                .flat_map(|cls| {
                    [
                        Bonus::new(
                            Self::Religious.attribute(),
                            (cls.level().to_value() + val!(1)) / val!(2),
                            BonusType::Stacking,
                            cls.level(),
                        )
                        .with_show_condition(cls.level().to_value().is_some()),
                        Bonus::new(
                            Self::Wilderness.attribute(),
                            (cls.level().to_value() + val!(1)) / val!(2),
                            BonusType::Stacking,
                            cls.level(),
                        )
                        .with_show_condition(cls.level().to_value().is_some()),
                    ]
                }),
        )
    }
}
