//! Describes different toggle groups used for toggles that are exclusive with other toggle groups

use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::{epic_sphere::EpicSphere, toggle::Toggle};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Clone, Copy)]
/// Describes different toggle groups within the game. Each toggle group can only have one toggle
/// at a time.
///
/// To do this, we are using the "Toggle Group" as the source of the bonus. Whenever we change the
/// toggle group, the Breakdowns object will remove any bonuses that turn on any toggles of the
/// same type.
///
/// While this solution is a little weird, as it abuses some mechanics, this is the simplest
/// solution until a more integrated solution can be resolved.
pub enum ToggleGroup {
    /// Individual
    Toggle(Toggle),
    /// Defensive combat stance
    DefensiveCombat,
    /// Monk Stance
    MonkStance,
    /// Major Form
    MajorForm,
    /// Warlock Pact
    WarlockPact,
    /// Epic Past Life
    EpicPastLife(EpicSphere),
    /// Iconic Past Life
    IconicPastLife,
}

impl Display for ToggleGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Toggle(toggle) => write!(f, "Toggle: {toggle}"),
            Self::DefensiveCombat => write!(f, "Defensive Combat"),
            Self::MonkStance => write!(f, "Monk Stance"),
            Self::MajorForm => write!(f, "Major Form"),
            Self::WarlockPact => write!(f, "Warlock Pact"),
            Self::IconicPastLife => write!(f, "Iconic Past Life"),
            Self::EpicPastLife(sphere) => write!(f, "{sphere}"),
        }
    }
}
