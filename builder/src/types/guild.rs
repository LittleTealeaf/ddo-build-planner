//! Guild Attributes

use core::fmt;

use serde::{Deserialize, Serialize};
use utils::public_modules;

public_modules!(amenities);

/// Guild-focused attributes
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Guild {
    /// Guild Level
    Level,
    /// Guild Amenities
    Amenity(GuildAmenity),
}

impl fmt::Display for Guild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Level => write!(f, "Guild Level"),
            Self::Amenity(a) => write!(f, "Guild Amenity: {a}"),
        }
    }
}
