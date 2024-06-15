use core::fmt;

use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use super::{ToToggle, Toggle};

/// Guild Amenities
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Clone, Copy)]
pub enum GuildAmenity {
    /// Sign of the Silver Flame I
    #[serde(rename = "sotsf1", alias = "SignOfTheSilverFlameI")]
    SignOfTheSilverFlameI,
    /// Shrine to the Devourer I
    #[serde(rename = "sttd1", alias = "ShrineToTheDevourerI")]
    ShrineToTheDevourerI,
    /// Stormreaver Memorial I
    #[serde(rename = "sm1", alias = "StormreaverMemorialI")]
    StormreaverMemorialI,
    /// Shrine of Experience I
    #[serde(rename = "soe1", alias = "ShrineOfExperienceI")]
    ShrineOfExperienceI,
    /// The Orien Express
    #[serde(rename = "toe", alias = "TheOrienExpress")]
    TheOrienExpress,
    /// Three-Finger Thad's
    #[serde(rename = "tft", alias = "ThreeFingerThads")]
    ThreeFingerThads,
    /// Farshifter's Chambers
    #[serde(rename = "fc", alias = "FarshiftersChambers")]
    FarshiftersChambers,
    /// Chronoscope
    #[serde(rename = "c", alias = "Chronoscope")]
    Chronoscope,
    /// Sellsword's Tavern
    #[serde(rename = "st", alias = "SellswordsTavern")]
    SellswordsTavern,
    /// Bath House
    #[serde(rename = "bho", alias = "BathHouse")]
    BathHouse,
    /// Floating Rock Garden
    #[serde(rename = "frg", alias = "FloatingRockGarden")]
    FloatingRockGarden,
    /// Paradoxical Puzzle Box
    #[serde(rename = "ppb", alias = "ParadoxicalPuzzleBox")]
    ParadoxicalPuzzleBox,
    /// Old Sully's Grog Cellar
    #[serde(rename = "osgc", alias = "OldSullysGrogCellar")]
    OldSullysGrogCellar,
    /// Throne Room
    #[serde(rename = "tr", alias = "ThroneRoom")]
    ThroneRoom,
    /// Guild Storage I
    #[serde(rename = "gs1", alias = "GuildStorageI")]
    GuildStorageI,
    /// Shrine of Experience II
    #[serde(rename = "soe2", alias = "ShrineOfExperienceII")]
    ShrineOfExperienceII,
    /// Tactical Training Room
    #[serde(rename = "ttr", alias = "TacticalTrainingRoom")]
    TacticalTrainingRoom,
    /// Danger Room
    #[serde(rename = "dr", alias = "DangerRoom")]
    DangerRoom,
    /// Forbidden Library
    #[serde(rename = "fl", alias = "ForbiddenLibrary")]
    ForbiddenLibrary,
    /// Archery Range
    #[serde(rename = "ar", alias = "ArcheryRange")]
    ArcheryRange,
    /// Armory
    #[serde(rename = "am", alias = "Armory")]
    Armory,
    /// Otto's Irresistable Dancehall
    #[serde(rename = "oid", alias = "OttosIrresistableDancehall")]
    OttosIrresistableDancehall,
    /// Crusader's Chapel
    #[serde(rename = "cc", alias = "CrusadersChapel")]
    CrusadersChapel,
    /// Arcane Sanctum
    #[serde(rename = "as", alias = "ArcaneSanctum")]
    ArcaneSanctum,
    /// Trapsmith's Workshop
    #[serde(rename = "tw", alias = "TrapsmithsWorkshop")]
    TrapsmithsWorkshop,
    /// Shrine of Experience III
    #[serde(rename = "soe3", alias = "ShrineOfExperienceIII")]
    ShrineOfExperienceIII,
    /// Wild Grove
    #[serde(rename = "wg", alias = "WildGrove")]
    WildGrove,
    /// Grandmaster's Dojo
    #[serde(rename = "gd", alias = "GrandmastersDojo")]
    GrandmastersDojo,
    /// Proving Ground
    #[serde(rename = "pg", alias = "ProvingGround")]
    ProvingGround,
    /// Collegium of the Twelve
    #[serde(rename = "cott", alias = "CollegiumOfTheTwelve")]
    CollegiumOfTheTwelve,
    /// Bash the Breakables Cargo Bay
    #[serde(rename = "btbcb", alias = "BashTheBreakablesCargoBay")]
    BashTheBreakablesCargoBay,
    /// Black Abbot's Shadow
    #[serde(rename = "bas", alias = "BlackAbbotsShadow")]
    BlackAbbotsShadow,
    /// Banquet Hall
    #[serde(rename = "bh", alias = "BanquetHall")]
    BanquetHall,
    /// Concert Hall
    #[serde(rename = "ch", alias = "ConcertHall")]
    ConcertHall,
    /// Archwizard
    #[serde(rename = "a", alias = "Archwizard")]
    Archwizard,
    /// Green Steel Crafting Hall
    #[serde(rename = "gsch", alias = "GreenSteelCraftingHall")]
    GreenSteelCraftingHall,
    /// Shrine of Experience IV
    #[serde(rename = "soh5", alias = "ShrineOfExperienceIV")]
    ShrineOfExperienceIV,
    /// Cannith Crafting Hall
    #[serde(rename = "cch", alias = "CannithCraftingHall")]
    CannithCraftingHall,
    /// Game Hunter
    #[serde(rename = "gm", alias = "GameHunter")]
    GameHunter,
    /// Fencing Master
    #[serde(rename = "fm", alias = "FencingMaster")]
    FencingMaster,
    /// Ninja Assassin
    #[serde(rename = "na", alias = "NinjaAssassin")]
    NinjaAssassin,
    /// Hag Apothecary
    #[serde(rename = "ha", alias = "HagApothecary")]
    HagApothecary,
    /// Guild Storage II
    #[serde(rename = "gs2", alias = "GuildStorageII")]
    GuildStorageII,
    /// Grand Reliquary I
    #[serde(rename = "gr1", alias = "GrandReliquaryI")]
    GrandReliquaryI,
    /// Shrine of Experience V
    #[serde(rename = "soe5", alias = "ShrineOfExperienceV")]
    ShrineOfExperienceV,
    /// Sign of the Silver Flame II
    #[serde(rename = "sotsf2", alias = "SignOfTheSilverFlameII")]
    SignOfTheSilverFlameII,
    /// Shrine to the Devourer II
    #[serde(rename = "sttd2", alias = "ShrineToTheDevourerII")]
    ShrineToTheDevourerII,
    /// Guild Storage III
    #[serde(rename = "gs3", alias = "GuildStorageIII")]
    GuildStorageIII,
    /// Stormreaver Memorial II
    #[serde(rename = "sm2", alias = "StormreaverMemorialII")]
    StormreaverMemorialII,
    /// Grand Reliquary II
    #[serde(rename = "gr2", alias = "GrandReliquaryII")]
    GrandReliquaryII,
    /// Sign of the Silver Flame III
    #[serde(rename = "sotsf3", alias = "SignOfTheSilverFlameIII")]
    SignOfTheSilverFlameIII,
    /// Shrine to the Devourer III
    #[serde(rename = "sttd3", alias = "ShrineToTheDevourerIII")]
    ShrineToTheDevourerIII,
    /// Guild Storage IV
    #[serde(rename = "gs4", alias = "GuildStorageIV")]
    GuildStorageIV,
    /// Stormreaver Memorial III
    #[serde(rename = "sm3", alias = "StormreaverMemorialIII")]
    StormreaverMemorialIII,
    /// Grand Reliquary III
    #[serde(rename = "gr3", alias = "GrandReliquaryIII")]
    GrandReliquaryIII,
    /// Guild Storage V
    #[serde(rename = "gs5", alias = "GuildStorageV")]
    GuildStorageV,
    /// Sign of the Silver Flame IV
    #[serde(rename = "sotsf4", alias = "SignOfTheSilverFlameIV")]
    SignOfTheSilverFlameIV,
    /// Shrine to the Devourer IV
    #[serde(rename = "sttd4", alias = "ShrineToTheDevourerIV")]
    ShrineToTheDevourerIV,
    /// Stormreaver Memorial IV
    #[serde(rename = "sm4", alias = "StormreaverMemorialIV")]
    StormreaverMemorialIV,
    /// Grand Reliquary IV
    #[serde(rename = "gr4", alias = "GrandReliquaryIV")]
    GrandReliquaryIV,
}

impl GuildAmenity {
    /// All guild amenities
    pub const ALL: [Self; 60] = [
        Self::SignOfTheSilverFlameI,
        Self::ShrineToTheDevourerI,
        Self::StormreaverMemorialI,
        Self::ShrineOfExperienceI,
        Self::TheOrienExpress,
        Self::ThreeFingerThads,
        Self::FarshiftersChambers,
        Self::Chronoscope,
        Self::SellswordsTavern,
        Self::BathHouse,
        Self::FloatingRockGarden,
        Self::ParadoxicalPuzzleBox,
        Self::OldSullysGrogCellar,
        Self::ThroneRoom,
        Self::GuildStorageI,
        Self::ShrineOfExperienceII,
        Self::TacticalTrainingRoom,
        Self::DangerRoom,
        Self::ForbiddenLibrary,
        Self::ArcheryRange,
        Self::Armory,
        Self::OttosIrresistableDancehall,
        Self::CrusadersChapel,
        Self::ArcaneSanctum,
        Self::TrapsmithsWorkshop,
        Self::ShrineOfExperienceIII,
        Self::WildGrove,
        Self::GrandmastersDojo,
        Self::ProvingGround,
        Self::CollegiumOfTheTwelve,
        Self::BashTheBreakablesCargoBay,
        Self::BlackAbbotsShadow,
        Self::BanquetHall,
        Self::ConcertHall,
        Self::Archwizard,
        Self::GreenSteelCraftingHall,
        Self::ShrineOfExperienceIV,
        Self::CannithCraftingHall,
        Self::GameHunter,
        Self::FencingMaster,
        Self::NinjaAssassin,
        Self::HagApothecary,
        Self::GuildStorageII,
        Self::GrandReliquaryI,
        Self::ShrineOfExperienceV,
        Self::SignOfTheSilverFlameII,
        Self::ShrineToTheDevourerII,
        Self::GuildStorageIII,
        Self::StormreaverMemorialII,
        Self::GrandReliquaryII,
        Self::SignOfTheSilverFlameIII,
        Self::ShrineToTheDevourerIII,
        Self::GuildStorageIV,
        Self::StormreaverMemorialIII,
        Self::GrandReliquaryIII,
        Self::GuildStorageV,
        Self::SignOfTheSilverFlameIV,
        Self::ShrineToTheDevourerIV,
        Self::StormreaverMemorialIV,
        Self::GrandReliquaryIV,
    ];
}

impl fmt::Display for GuildAmenity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SignOfTheSilverFlameI => write!(f, "Sign of the Silver Flame I"),
            Self::ShrineToTheDevourerI => write!(f, "Shrine to the Devourer I"),
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
            Self::ShrineToTheDevourerIV => write!(f, "Shrine to the Devourer IV"),
            Self::StormreaverMemorialIV => write!(f, "Stormreaver Memorial IV"),
            Self::GrandReliquaryIV => write!(f, "Grand Reliquary IV"),
        }
    }
}

impl ToToggle for GuildAmenity {
    fn to_toggle(self) -> Toggle {
        Toggle::Guild(self)
    }
}

impl StaticValues for GuildAmenity {
    fn values() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }
}
