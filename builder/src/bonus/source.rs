use core::fmt;
use fmt::Display;

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use crate::{
    attribute::{Attribute, ToAttribute},
    types::{slider::Slider, toggle_group::ToggleGroup},
};

/// Dictates the source that a bonus comes from.
///
/// Each bonus must have a source that dictates where that bonus came from. For example, if an attribute returns any new bonuses, they must all have a source of that attribute.
#[derive(Hash, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BonusSource {
    /// Indicates that the bonus comes from an attribute.
    Attribute(Attribute),
    /// Toggle Group Specific
    ToggleGroup(ToggleGroup),
    /// Individual sliders
    Slider(Slider),
    /// Dictates any custom bonuses for general uses. When possible, do not use this source
    Custom(String),
    /// Used for debugging purposes.
    Debug(usize),
    /// Only used for initial values
    Base,
    /// Used for the editor Sandbox mode
    Sandbox,
}

impl Display for BonusSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Slider(slider) => write!(f, "Slider: {slider}"),
            Self::Attribute(attr) => write!(f, "Attribute: {attr}"),
            Self::Custom(string) => write!(f, "{string}"),
            Self::Debug(num) => write!(f, "Debug: {num}"),
            Self::Base => write!(f, "Base"),
            Self::ToggleGroup(group) => write!(f, "Toggle Group: {group}"),
            Self::Sandbox => write!(f, "Sandbox"),
        }
    }
}

impl From<Attribute> for BonusSource {
    fn from(value: Attribute) -> Self {
        Self::Attribute(value)
    }
}

impl From<String> for BonusSource {
    fn from(value: String) -> Self {
        Self::Custom(value)
    }
}

impl<T> From<T> for BonusSource
where
    T: ToAttribute,
{
    fn from(value: T) -> Self {
        value.to_attribute().into()
    }
}

/// Trait that provides the [`ToSource::to_source`] method
pub trait ToSource {
    /// Converts this into a [`BonusSource`].
    /// Acts as a shortcut for [`BonusSource::from`]
    fn to_source(self) -> BonusSource;
}

impl<T> ToSource for T
where
    BonusSource: From<T>,
{
    fn to_source(self) -> BonusSource {
        BonusSource::from(self)
    }
}

impl StaticValues for BonusSource {
    fn values() -> impl Iterator<Item = Self> {
        chain!(
            [Self::Base, Self::Sandbox],
            Attribute::values().map(Self::Attribute),
            Slider::values().map(Self::Slider)
        )
    }
}
