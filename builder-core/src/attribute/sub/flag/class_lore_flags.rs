use enum_map::Enum;
use serde::{Deserialize, Serialize};

/// Any flags that interact with Class Lore
#[derive(Copy, Clone, PartialEq, Eq, Hash, Enum, Serialize, Deserialize, Debug)]
pub enum ClassLoreFlag {
    /// Provides bonuses to magical sheltering equal to their religious lore
    ReligiousLoreToQualityMagicalSheltering,
    /// Provides bonuses to physical sheltering equal to their religious lore
    ReligiousLoreToQualityPhysicalSheltering,
}

impl ToString for ClassLoreFlag {
    fn to_string(&self) -> String {
        match self {
            ClassLoreFlag::ReligiousLoreToQualityMagicalSheltering => {
                String::from("Religious Lore to Quality Magical Sheltering")
            }
            ClassLoreFlag::ReligiousLoreToQualityPhysicalSheltering => {
                String::from("Religious Lore to Quality Physical Sheltering")
            }
        }
    }
}
