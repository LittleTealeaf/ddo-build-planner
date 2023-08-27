use serde::{Deserialize, Serialize};

/// Describes different types of damage possible in Dungeons & Dragons Online
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DamageType {}
