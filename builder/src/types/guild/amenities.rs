use core::fmt;

use rust_decimal::prelude::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{BonusTemplate, BonusType, ToValue, Value},
    types::{
        ability::Ability, damage_type::DamageType, guild::Guild, heal_amp::HealingAmplification,
        saving_throw::SavingThrow, skill::Skill,
    },
    val,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Clone, Copy)]
pub enum GuildAmenity {
    /// Sign of the Silver Flame I
    SignOfTheSilverFlameI,
    /// Shrine of the Devourer I
    ShrineOfTheDevourerI,
    /// Stormreaver Memorial I
    StormreaverMemorialI,
    /// Shrine of Experience I
    ShrineOfExperienceI,
    /// The Orien Express
    TheOrienExpress,
    /// Three-Finger Thad's
    ThreeFingerThads,
    /// Farshifter's Chambers
    FarshiftersChambers,
    /// Chronoscope
    Chronoscope,
    /// Sellsword's Tavern
    SellswordsTavern,
    /// Bath House
    BathHouse,
    /// Floating Rock Garden
    FloatingRockGarden,
    /// Paradoxical Puzzle Box
    ParadoxicalPuzzleBox,
    /// Old Sully's Grog Cellar
    OldSullysGrogCellar,
    /// Throne Room
    ThroneRoom,
    /// Guild Storage I
    GuildStorageI,
    /// Shrine of Experience II
    ShrineOfExperienceII,
    /// Tactical Training Room
    TacticalTrainingRoom,
    /// Danger Room
    DangerRoom,
    /// Forbidden Library
    ForbiddenLibrary,
    /// Archery Range
    ArcheryRange,
    /// Armory
    Armory,
    /// Otto's Irresistable Dancehall
    OttosIrresistableDancehall,
    /// Crusader's Chapel
    CrusadersChapel,
    /// Arcane Sanctum
    ArcaneSanctum,
    /// Trapsmith's Workshop
    TrapsmithsWorkshop,
    /// Shrine of Experience III
    ShrineOfExperienceIII,
    /// Wild Grove
    WildGrove,
    /// Grandmaster's Dojo
    GrandmastersDojo,
    /// Proving Ground
    ProvingGround,
    /// Collegium of the Twelve
    CollegiumOfTheTwelve,
    /// Bash the Breakables Cargo Bay
    BashTheBreakablesCargoBay,
    /// Black Abbot's Shadow
    BlackAbbotsShadow,
    /// Banquet Hall
    BanquetHall,
    /// Concert Hall
    ConcertHall,
    /// Archwizard
    Archwizard,
    /// Green Steel Crafting Hall
    GreenSteelCraftingHall,
    /// Shrine of Experience IV
    ShrineOfExperienceIV,
    /// Cannith Crafting Hall
    CannithCraftingHall,
    /// Game Hunter
    GameHunter,
    /// Fencing Master
    FencingMaster,
    /// Ninja Assassin
    NinjaAssassin,
    /// Hag Apothecary
    HagApothecary,
    /// Guild Storage II
    GuildStorageII,
    /// Grand Reliquary I
    GrandReliquaryI,
    /// Shrine of Experience V
    ShrineOfExperienceV,
    /// Sign of the Silver Flame II
    SignOfTheSilverFlameII,
    /// Shrine to the Devourer II
    ShrineToTheDevourerII,
    /// Guild Storage III
    GuildStorageIII,
    /// Stormreaver Memorial II
    StormreaverMemorialII,
    /// Grand Reliquary II
    GrandReliquaryII,
    /// Sign of the Silver Flame III
    SignOfTheSilverFlameIII,
    /// Shrine to the Devourer III
    ShrineToTheDevourerIII,
    /// Guild Storage IV
    GuildStorageIV,
    /// Stormreaver Memorial III
    StormreaverMemorialIII,
    /// Grand Reliquary III
    GrandReliquaryIII,
    /// Guild Storage V
    GuildStorageV,
    /// Sign of the Silver Flame IV
    SignOfTheSilverFlameIV,
    /// Shrine of the Devourer IV
    ShrineOfTheDevourerIV,
    /// Stormreaver Memorial IV
    StormreaverMemorialIV,
    /// Grand Reliquary IV
    GrandReliquaryIV,
}

impl GetBonuses for GuildAmenity {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        fn scale_with_level<A, B, C>(a: A, b: B, c: C) -> Value
        where
            A: Into<Value>,
            B: Into<Value>,
            C: Into<Value>,
        {
            Value::condition(
                Attribute::TotalCharacterLevel
                    .to_value()
                    .greater_than(val!(20)),
                c,
                Value::condition(
                    Attribute::TotalCharacterLevel
                        .to_value()
                        .greater_than(val!(10)),
                    b,
                    a,
                ),
            )
        }

        fn resist_spell_power<I>(types: I) -> impl Iterator<Item = BonusTemplate>
        where
            I: IntoIterator<Item = DamageType>,
        {
            types
                .into_iter()
                .flat_map(|dt| [Attribute::spell_power(dt), Attribute::Resistance(dt)])
                .map(|attribute| {
                    BonusTemplate::new(
                        attribute,
                        BonusType::Guild,
                        scale_with_level(val!(5), val!(10), val!(15)),
                    )
                })
        }

        fn ability_bonuses<I>(abilities: I) -> impl Iterator<Item = BonusTemplate>
        where
            I: IntoIterator<Item = Ability>,
        {
            abilities
                .into_iter()
                .map(|ability| BonusTemplate::new(ability, BonusType::Guild, val!(2)))
        }

        fn skill_bonus<I>(skills: I) -> impl Iterator<Item = BonusTemplate>
        where
            I: IntoIterator<Item = Skill>,
        {
            skills.into_iter().map(|skill| {
                BonusTemplate::new(
                    skill,
                    BonusType::Guild,
                    scale_with_level(val!(1), val!(2), val!(3)),
                )
            })
        }

        match self {
            Self::SignOfTheSilverFlameI => Some(
                [
                    Attribute::spell_power(DamageType::Light),
                    Attribute::spell_power(DamageType::Fire),
                    Attribute::spell_power(DamageType::Alignment),
                    Attribute::Resistance(DamageType::Fire),
                ]
                .map(|attribute| {
                    BonusTemplate::new(
                        attribute,
                        BonusType::Guild,
                        scale_with_level(val!(5), val!(10), val!(15)),
                    )
                })
                .to_vec(),
            ),
            Self::ShrineOfTheDevourerI => {
                Some(resist_spell_power([DamageType::Acid, DamageType::Cold]).collect())
            }
            Self::StormreaverMemorialI => {
                Some(resist_spell_power([DamageType::Sonic, DamageType::Electric]).collect())
            }
            Self::Chronoscope => Some(vec![
                BonusTemplate::new(
                    SavingThrow::Reflex,
                    BonusType::Guild,
                    scale_with_level(val!(1), val!(2), val!(3)),
                ),
                // TODO: only when in public spaces
                BonusTemplate::new(Attribute::MovementSpeed, BonusType::Enhancement, val!(40)),
            ]),
            Self::BathHouse => Some(vec![
                BonusTemplate::new(HealingAmplification::Positive, BonusType::Guild, val!(20)),
                // TODO: uncon range +5/10/15
                // TODO: -10% damage when helpless
            ]),
            Self::FloatingRockGarden => {
                Some(ability_bonuses([Ability::Strength, Ability::Wisdom]).collect())
            }
            Self::ParadoxicalPuzzleBox => {
                Some(ability_bonuses([Ability::Dexterity, Ability::Intelligence]).collect())
            }
            Self::OldSullysGrogCellar => {
                Some(ability_bonuses([Ability::Charisma, Ability::Constitution]).collect())
            }
            Self::ThroneRoom => Some(
                skill_bonus([
                    Skill::Bluff,
                    Skill::Diplomacy,
                    Skill::Haggle,
                    Skill::Intimidate,
                    Skill::Listen,
                ])
                .collect(),
            ),
            _ => None,
        }
    }
}

impl fmt::Display for GuildAmenity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SignOfTheSilverFlameI => write!(f, "Sign of the Silver Flame I"),
            Self::ShrineOfTheDevourerI => write!(f, "Shrine of the Devourer I"),
            Self::StormreaverMemorialI => write!(f, "Stormreaver Memorial I"),
            Self::ShrineOfExperienceI => write!(f, "Shrine of Experience I"),
            Self::TheOrienExpress => write!(f, "The Orien Express"),
            Self::ThreeFingerThads => write!(f, "Three Finger Thad's"),
            Self::FarshiftersChambers => write!(f, "Farshifter's Chambers"),
            Self::Chronoscope => write!(f, "Chronoscope"),
            Self::SellswordsTavern => write!(f, "Sellsword's Tavern"),
            Self::BathHouse => write!(f, "Bath House"),
            Self::FloatingRockGarden => write!(f, "Floating Rock Garden"),
            Self::ParadoxicalPuzzleBox => write!(f, "Paradoxical Puzzle Box"),
            Self::OldSullysGrogCellar => write!(f, "Old Sully's Grog Cellar"),
            Self::ThroneRoom => write!(f, "Throne Room"),
            Self::GuildStorageI => write!(f, "Guild Storage I"),
            Self::ShrineOfExperienceII => write!(f, "Shrine of Experience II"),
            Self::TacticalTrainingRoom => write!(f, "Tactical Training Room"),
            Self::DangerRoom => write!(f, "Danger Room"),
            Self::ForbiddenLibrary => write!(f, "Forbidden Library"),
            Self::ArcheryRange => write!(f, "Archery Range"),
            Self::Armory => write!(f, "Armory"),
            Self::OttosIrresistableDancehall => write!(f, "Otto's Irresistible Dancehall"),
            Self::CrusadersChapel => write!(f, "Crusader's Chapel"),
            Self::ArcaneSanctum => write!(f, "Arcane Sanctum"),
            Self::TrapsmithsWorkshop => write!(f, "Trapsmith's Workshop"),
            Self::ShrineOfExperienceIII => write!(f, "Shrine of Experience III"),
            Self::WildGrove => write!(f, "Wild Grove"),
            Self::GrandmastersDojo => write!(f, "Grandmaster's Dojo"),
            Self::ProvingGround => write!(f, "Proving Ground"),
            Self::CollegiumOfTheTwelve => write!(f, "Collegium of the Twelve"),
            Self::BashTheBreakablesCargoBay => write!(f, "Bash the Breakables Cargo Bay"),
            Self::BlackAbbotsShadow => write!(f, "Black Abbot's Shadow"),
            Self::BanquetHall => write!(f, "Banquet Hall"),
            Self::ConcertHall => write!(f, "Concert Hall"),
            Self::Archwizard => write!(f, "Archwizard"),
            Self::GreenSteelCraftingHall => write!(f, "Green Steel Crafting Hall"),
            Self::ShrineOfExperienceIV => write!(f, "Shrine of Experience IV"),
            Self::CannithCraftingHall => write!(f, "Cannith Crafting Hall"),
            Self::GameHunter => write!(f, "Game Hunter"),
            Self::FencingMaster => write!(f, "Fencing Master"),
            Self::NinjaAssassin => write!(f, "Ninja Assassin"),
            Self::HagApothecary => write!(f, "Hag Apothecary"),
            Self::GuildStorageII => write!(f, "Guild Storage II"),
            Self::GrandReliquaryI => write!(f, "Grand Reliquary I"),
            Self::ShrineOfExperienceV => write!(f, "Shrine of Experience V"),
            Self::SignOfTheSilverFlameII => write!(f, "Sign of the Silver Flame II"),
            Self::ShrineToTheDevourerII => write!(f, "Shrine to the Devourer II"),
            Self::GuildStorageIII => write!(f, "Guild Storage III"),
            Self::StormreaverMemorialII => write!(f, "Stormreaver Memorial II"),
            Self::GrandReliquaryII => write!(f, "Grand Reliquary II"),
            Self::SignOfTheSilverFlameIII => write!(f, "Sign of the Silver Flame III"),
            Self::ShrineToTheDevourerIII => write!(f, "Shrine to the Devourer III"),
            Self::GuildStorageIV => write!(f, "Guild Storage IV"),
            Self::StormreaverMemorialIII => write!(f, "Stormreaver Memorial III"),
            Self::GrandReliquaryIII => write!(f, "Grand Reliquary III"),
            Self::GuildStorageV => write!(f, "Guild Storage V"),
            Self::SignOfTheSilverFlameIV => write!(f, "Sign of the Silver Flame IV"),
            Self::ShrineOfTheDevourerIV => write!(f, "Shrine of the Devourer IV"),
            Self::StormreaverMemorialIV => write!(f, "Stormreaver Memorial IV"),
            Self::GrandReliquaryIV => write!(f, "Grand Reliquary IV"),
        }
    }
}
