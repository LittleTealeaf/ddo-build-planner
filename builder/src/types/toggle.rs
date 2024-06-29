//! Any attribute that requires the user to interact / configure

public_modules!(attacking_target, guild_amenities, seasonal_affinity);

use core::fmt::{self, Display};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utils::{chain_tree, enums::StaticValues, public_modules};

use crate::{
    attribute::{Attribute, GetBonuses, ToAttribute},
    bonus::{Bonus, BonusSource, BonusTemplate, BonusType},
    feat::{EpicPastLife, IconicPastLife},
};

use super::{
    flag::{Flag, ToFlag},
    toggle_group::ToggleGroup,
};

/// Toggles are interactive elements that the user is able to interact with to modify the "current state" of the character.
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Toggle {
    /// Is the character blocking
    #[serde(rename = "bl", alias = "Blocking")]
    Blocking,
    /// Is the character in reaper mode
    #[serde(rename = "r", alias = "InReaper")]
    InReaper,
    /// Is the character attacking a certain target
    #[serde(rename = "at", alias = "Attacking")]
    Attacking(AttackingTarget),
    /// Is the character sneak attacking.
    #[serde(rename = "sa", alias = "SneakAttack")]
    SneakAttack,
    /// Iconic Past Life
    #[serde(rename = "ipl", alias = "IconicPastLife")]
    IconicPastLife(IconicPastLife),
    /// Epic Past Life
    #[serde(rename = "epl", alias = "EpicPastLife")]
    EpicPastLife(EpicPastLife),
    /// Is the user flanking the enemy
    #[serde(rename = "fla", alias = "Flanking")]
    Flanking,
    /// Guild Amenities
    #[serde(rename = "gb", alias = "Guild")]
    Guild(GuildAmenity),
    /// Seasonal Affinity
    #[serde(rename = "eladrin", alias = "SeasonalAffinity")]
    SeasonalAffinity(SeasonalAffinity),
}
// TODO: Make a sub-toggle for "Attacking" (such as attacking a certain type of enemy)

impl Toggle {
    /// Returns the toggle source used to enable this toggle
    #[must_use]
    pub fn toggl_source(&self) -> BonusSource {
        BonusSource::ToggleGroup(
            self.custom_toggle_group()
                .unwrap_or(ToggleGroup::Toggle(*self)),
        )
    }

    /// Creates a bonus that either enables or disables this toggle
    #[must_use]
    pub fn toggle_bonus(&self, enable: bool) -> Bonus {
        Bonus::new(
            self.to_attribute(),
            BonusType::Stacking,
            u8::from(enable),
            self.toggl_source(),
        )
    }
}

impl Display for Toggle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blocking => write!(f, "Blocking"),
            Self::InReaper => write!(f, "In Reaper"),
            Self::Attacking(target) => write!(f, "Attacking {target} Target"),
            Self::IconicPastLife(past_life) => write!(f, "{past_life}"),
            Self::EpicPastLife(past_life) => write!(f, "{past_life}"),
            Self::SneakAttack => write!(f, "Sneak Attack"),
            Self::Flanking => write!(f, "Flanking"),
            Self::Guild(amenity) => write!(f, "{amenity}"),
            Self::SeasonalAffinity(affinity) => write!(f, "{affinity} Affinity"),
        }
    }
}

impl GetBonuses<Self> for Toggle {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        (value > Decimal::ZERO).then(|| vec![BonusTemplate::toggle(*self)])
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

impl StaticValues for Toggle {
    fn values() -> impl Iterator<Item = Self> {
        chain_tree!(
            [Self::Blocking, Self::InReaper],
            AttackingTarget::values().map(Self::Attacking),
            IconicPastLife::values().map(Self::IconicPastLife),
            EpicPastLife::values().map(Self::EpicPastLife),
            SeasonalAffinity::values().map(Self::SeasonalAffinity),
        )
    }
}

/// Indicates that a toggle may have a toggle group that it must specifically entail
pub trait GetToggleGroup {
    /// Returns the toggle group for this toggle, if any
    fn custom_toggle_group(&self) -> Option<ToggleGroup>;
}

impl GetToggleGroup for Toggle {
    fn custom_toggle_group(&self) -> Option<ToggleGroup> {
        match self {
            Self::IconicPastLife(life) => life.custom_toggle_group(),
            Self::EpicPastLife(life) => life.custom_toggle_group(),
            Self::SeasonalAffinity(seasonal) => seasonal.custom_toggle_group(),
            _ => None,
        }
    }
}
