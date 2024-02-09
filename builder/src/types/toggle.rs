//! Any attribute that requires the user to interact / configure

public_modules!(attacking_target, iconic_past_life);

use core::fmt::{self, Display};

use itertools::chain;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utils::{enums::StaticOptions, public_modules};

use crate::{
    attribute::{Attribute, GetBonuses, ToAttribute},
    bonus::{BonusSource, BonusTemplate},
};

use self::iconic_past_life::IconicPastLife;

use super::{
    flag::{Flag, ToFlag},
    toggle_group::ToggleGroup,
};

/// Toggles are interactable elements that the user is able to interact with to modify the "current state" of the character.
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Toggle {
    /// Is the character blocking
    Blocking,
    /// Is the character in reaper mode
    InReaper,
    /// Is the character attacking a certain target
    Attacking(AttackingTarget),
    /// Iconic Past Life
    IconicPastLife(IconicPastLife),
}
// TODO: Make a sub-toggle for "Attacking" (such as attacking a certain type of enemy)

impl Display for Toggle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blocking => write!(f, "Blocking"),
            Self::InReaper => write!(f, "In Reaper"),
            Self::Attacking(target) => write!(f, "Attacking {target} Target"),
            Self::IconicPastLife(past_life) => write!(f, "{past_life}"),
        }
    }
}

impl GetBonuses<Self> for Toggle {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        (value > Decimal::ZERO).then(|| vec![BonusTemplate::toggle(*self, None)])
    }
}

impl ToAttribute for Toggle {
    fn to_attribute(self) -> Attribute {
        Attribute::Toggle(self)
    }
}

impl ToFlag for Toggle {
    fn to_flag(self) -> Flag {
        Flag::HasToggle(self)
    }
}

/// Indicates that this object is a toggle
pub trait ToToggle {
    /// Converts this to a toggle object
    fn to_toggle(self) -> Toggle;
}

impl<T> From<T> for Toggle
where
    T: ToToggle,
{
    fn from(value: T) -> Self {
        value.to_toggle()
    }
}

impl StaticOptions for Toggle {
    fn get_static() -> impl Iterator<Item = Self> {
        chain!(
            [Self::Blocking, Self::InReaper],
            AttackingTarget::get_static().map(Self::Attacking),
            IconicPastLife::get_static().map(Self::IconicPastLife),
        )
    }
}

/// Indicates that a toggle may have a toggle group that it must specifically entail
pub trait GetToggleGroup {
    /// Returns the toggle group for this toggle, if any
    fn toggle_group(&self) -> Option<ToggleGroup>;

    /// Returns SOME toggle source for the toggle
    fn get_toggle_source(&self, default_source: BonusSource) -> BonusSource {
        self.toggle_group()
            .map_or(default_source, BonusSource::ToggleGroup)
    }
}

impl GetToggleGroup for Toggle {
    fn toggle_group(&self) -> Option<ToggleGroup> {
        match self {
            Self::IconicPastLife(life) => life.toggle_group(),
            _ => None,
        }
    }
}
