use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, GetBonuses, GetCloned},
    bonus::Bonus,
};

/// Describes any interactable toggles that should be shown to the user.
///
/// This can be found in two attributes. The [`Flag::Toggle`] version indicates that this toggle
/// should be shown to the user. Then, interactions with the element will result in changes to the
/// [`Attribute::Toggle`] version.
///
/// [`Flag::Toggle`]: crate::attribute::Flag::Toggle
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Enum)]
pub enum Toggle {
    /// Indicates that the character is "holding shift" or blocking.
    Blocking,
    /// Indicates that the character is in reaper mode
    InReaper,
    /// Indicates that the character is currently attacking a tripped target.
    AttackingTrippedTarget,
}

impl ToString for Toggle {
    fn to_string(&self) -> String {
        match self {
            Toggle::Blocking => String::from("Blocking"),
            Toggle::InReaper => String::from("In Reaper"),
            Toggle::AttackingTrippedTarget => String::from("Attacking Tripped Target"),
        }
    }
}

impl GetBonuses for Toggle {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        None
    }
}

impl GetCloned for Toggle {
    fn get_cloned(&self) -> Option<Vec<Self>> {
        None
    }
}

impl From<Toggle> for Attribute {
    #[inline(always)]
    fn from(value: Toggle) -> Attribute {
        Attribute::Toggle(value)
    }
}
