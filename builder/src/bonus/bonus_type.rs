use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

/// Describes the stacking-type of a bonus.
///
/// Bonuses with the same [`BonusType`] will not stack, meaning that only the highest value will be
/// added. However, bonuses of different [`BonusType`] will stack.
///
/// Any bonus with a type of [`BonusType::Stacking`] will always stack no matter what.
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BonusType {
    /// Used in debugging bonus types
    Debug(u8),
    /// Bonuses that come from [`Attribute::AbilityModifier`]
    ///
    /// [`Attribute::AbilityModifier`]: crate::attribute::Attribute::AbilityModifier
    AbilityModifier,
    /// "Action Boost" bonuses
    ///
    /// These are typically bonuses that come from action boosts, or action boost-like buffs.
    #[serde(rename = "actbst", alias = "ActionBoost")]
    ActionBoost,
    /// Alchemical bonuses
    #[serde(rename = "alc", alias = "Alchemical")]
    Alchemical,
    /// Artifact bonuses
    ///
    /// These are typically bonuses from named item sets
    #[serde(rename = "art", alias = "Artifact")]
    Artifact,
    /// Competence bonuses
    #[serde(rename = "cmp", alias = "Competence")]
    Competence,
    /// Deflection bonus
    #[serde(rename = "dfl", alias = "Deflection")]
    Deflection,
    /// Dodge
    Dodge,
    /// Enhancement bonus
    #[serde(rename = "enh", alias = "Enhancement")]
    Enhancement,
    /// Epic bonus
    ///
    /// This bonus is typically used in epic destinies where different trees may provide bonuses to
    /// a particular attribute, but only the highest will count
    Epic,
    /// Exceptional bonus
    #[serde(rename = "exc", alias = "Exceptional")]
    Exceptional,
    /// Feat bonus
    Feat,
    /// Festive bonus
    Festive,
    /// Fortune bonus
    Fortune,
    /// Insightful bonus
    #[serde(rename = "ins", alias = "Insightful")]
    Insightful,
    /// Legendary bonus
    #[serde(rename = "leg", alias = "Legendary")]
    Legendary,
    /// Luck Bonus
    Luck,
    /// Morale bonus
    Morale,
    /// Music bonus
    Music,
    /// Primal bonus
    Primal,
    /// Profane bonus
    #[serde(rename = "pro", alias = "Profane")]
    Profane,
    /// Quality bonus
    #[serde(rename = "qual", alias = "Quality")]
    Quality,
    /// Racial bonus
    #[serde(rename = "race", alias = "Racial")]
    Racial,
    /// Sacred bonus
    #[serde(rename = "sac", alias = "Sacred")]
    Sacred,
    /// Size bonus
    Size,
    /// Shield bonus
    Shield,
    /// Stacking bonus
    ///
    /// This is the only [`BonusType`] that stacks with itself, since it is meant to be used
    /// whenever a bonus will stack with anything.
    ///
    /// This is also the default bonus, so it can be obtained via
    /// ```
    /// use builder::bonus::BonusType;
    /// let bonus_type: BonusType = BonusType::default();
    /// assert_eq!(BonusType::Stacking, bonus_type);
    /// ```
    #[serde(rename = "stck", alias = "Stacking")]
    Stacking,
    /// Used when things don't stack, but they don't really have a bonus type.
    #[serde(rename = "std", alias = "Standard")]
    Standard,
    /// Spooky type
    Spooky,
}

impl Display for BonusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Debug(channel) => write!(f, "Debug {channel}"),
            Self::AbilityModifier => write!(f, "Ability Modifier"),
            Self::ActionBoost => write!(f, "Action Boost"),
            Self::Alchemical => write!(f, "Alchemical"),
            Self::Artifact => write!(f, "Artifact"),
            Self::Competence => write!(f, "Competence"),
            Self::Deflection => write!(f, "Deflection"),
            Self::Enhancement => write!(f, "Enhancement"),
            Self::Epic => write!(f, "Epic"),
            Self::Exceptional => write!(f, "Exceptional"),
            Self::Feat => write!(f, "Feat"),
            Self::Festive => write!(f, "Festive"),
            Self::Fortune => write!(f, "Fortune"),
            Self::Insightful => write!(f, "Insightful"),
            Self::Legendary => write!(f, "Legendary"),
            Self::Morale => write!(f, "Morale"),
            Self::Music => write!(f, "Music"),
            Self::Primal => write!(f, "Primal"),
            Self::Profane => write!(f, "Profane"),
            Self::Quality => write!(f, "Quality"),
            Self::Sacred => write!(f, "Sacred"),
            Self::Shield => write!(f, "Shield"),
            Self::Size => write!(f, "Size"),
            Self::Stacking => write!(f, "Stacking"),
            Self::Spooky => write!(f, "Spooky"),
            Self::Standard => write!(f, "Standard"),
            Self::Racial => write!(f, "Racial"),
            Self::Dodge => write!(f, "Dodge"),
            Self::Luck => write!(f, "Luck"),
        }
    }
}

impl Default for BonusType {
    fn default() -> Self {
        Self::Stacking
    }
}

impl BonusType {
    /// Returns `true` if the bonus type is [`Stacking`]
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use builder::bonus::BonusType;
    ///
    /// assert!(BonusType::Stacking.is_stacking());
    /// assert!(!BonusType::Enhancement.is_stacking());
    /// ```
    ///
    /// [`Stacking`]: BonusType::Stacking
    #[must_use]
    pub const fn is_stacking(&self) -> bool {
        matches!(self, Self::Stacking)
    }
}

impl StaticOptions for BonusType {
    fn get_static() -> impl Iterator<Item = Self> {
        [
            Self::AbilityModifier,
            Self::ActionBoost,
            Self::Alchemical,
            Self::Artifact,
            Self::Competence,
            Self::Deflection,
            Self::Enhancement,
            Self::Epic,
            Self::Exceptional,
            Self::Feat,
            Self::Festive,
            Self::Fortune,
            Self::Insightful,
            Self::Legendary,
            Self::Morale,
            Self::Music,
            Self::Primal,
            Self::Profane,
            Self::Quality,
            Self::Sacred,
            Self::Shield,
            Self::Size,
            Self::Stacking,
            Self::Spooky,
            Self::Standard,
            Self::Racial,
            Self::Dodge,
            Self::Luck,
        ]
        .into_iter()
    }
}
