//! Describes different toggle groups used for toggles that are exclusive with other toggle groups

use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};

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
    /// Defensive combat stance
    DefensiveCombat,
    /// Monk Stance
    MonkStance,
    /// Major Form
    MajorForm,
    /// Warlock Pact
    WarlockPact,
    /// Iconic Past Life
    IconicPastLife,
    /// Divine Past Life
    DivinePastLife,
    /// Martial Past Life
    MartialPastLife,
    /// Primal Past Life
    PrimalPastLife,
    /// Arcane Past Life
    ArcanePastLife,
}

impl Display for ToggleGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DefensiveCombat => write!(f, "Defensive Combat"),
            Self::MonkStance => write!(f, "Monk Stance"),
            Self::MajorForm => write!(f, "Major Form"),
            Self::WarlockPact => write!(f, "Warlock Pact"),
            Self::IconicPastLife => write!(f, "Iconic Past Life"),
            Self::DivinePastLife => write!(f, "Divine Past Life"),
            Self::MartialPastLife => write!(f, "Martial Past Life"),
            Self::PrimalPastLife => write!(f, "Primal Past Life"),
            Self::ArcanePastLife => write!(f, "Arcane Past Life"),
        }
    }
}
