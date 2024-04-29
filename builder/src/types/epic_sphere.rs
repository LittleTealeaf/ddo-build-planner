use std::fmt::Display;

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
    pub const ALL: [Self; 4] = [
        Self::Arcane,
        Self::Primal,
        Self::Divine,
        Self::Martial
    ];
}

impl Display for EpicSphere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EpicSphere::Arcane => write!(f, "Arcane Sphere"),
            EpicSphere::Primal => write!(f, "Primal Sphere"),
            EpicSphere::Divine => write!(f, "Divine Sphere"),
            EpicSphere::Martial => write!(f, "Martial Sphere"),
        }
    }
}

impl StaticOptions for EpicSphere {
    fn get_static() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }
}
