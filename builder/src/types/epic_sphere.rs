//! Epic Spheres
use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

/// Epic Past Life Sphere.
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EpicSphere {
    /// Arcane Sphere
    Arcane,
    /// Primal Sphere
    Primal,
    /// Divien Sphere
    Divine,
    /// Martial Sphere
    Martial,
}

impl EpicSphere {
    /// All of the epic spheres
    pub const VALUES: [Self; 4] = [Self::Arcane, Self::Primal, Self::Divine, Self::Martial];
}

impl Display for EpicSphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Arcane => write!(f, "Arcane Sphere"),
            Self::Primal => write!(f, "Primal Sphere"),
            Self::Divine => write!(f, "Divine Sphere"),
            Self::Martial => write!(f, "Martial Sphere"),
        }
    }
}

impl StaticOptions for EpicSphere {
    fn get_static() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }
}
