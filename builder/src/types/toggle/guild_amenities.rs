use core::fmt;

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use super::{ToToggle, Toggle};

/// Guild Amenities
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Clone, Copy)]
pub enum GuildAmenity {
    /// Sign of the Silver Flame I
    SignOfTheSilverFlameI,
    /// Shrine to the Devourer I
    ShrineToTheDevourerI,
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
    /// Shrine to the Devourer IV
    ShrineToTheDevourerIV,
    /// Stormreaver Memorial IV
    StormreaverMemorialIV,
    /// Grand Reliquary IV
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

impl StaticOptions for GuildAmenity {
    fn get_static() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }
}
