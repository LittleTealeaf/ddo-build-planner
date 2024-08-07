use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

/// Describes the stacking-type of a bonus.
///
/// Bonuses with the same [`BonusType`] will not stack, meaning that only the highest value will be
/// added. However, bonuses of different [`BonusType`] will stack.
///
/// Any bonus with a type of [`BonusType::Stacking`] will always stack no matter what.
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BonusType {
    /// Used in debugging bonus types
    #[serde(rename = "d", alias = "Debug")]
    Debug(usize),
    /// Bonuses that come from [`Attribute::AbilityModifier`]
    ///
    /// [`Attribute::AbilityModifier`]: crate::attribute::Attribute::AbilityModifier
    #[serde(rename = "am", alias = "AbilityModifier")]
    AbilityModifier,
    /// "Action Boost" bonuses
    ///
    /// These are typically bonuses that come from action boosts, or action boost-like buffs.
    #[serde(rename = "actbst", alias = "ActionBoost")]
    ActionBoost,
    /// Alchemical bonuses
    #[serde(rename = "al", alias = "alc", alias = "Alchemical")]
    Alchemical,
    /// Artifact bonuses
    ///
    /// These are typically bonuses from named item sets
    #[serde(rename = "ar", alias = "art", alias = "Artifact")]
    Artifact,
    /// Competence bonuses
    #[serde(rename = "cm", alias = "cmp", alias = "Competence")]
    Competence,
    /// Deflection bonus
    #[serde(rename = "dfl", alias = "Deflection")]
    Deflection,
    /// Dodge
    #[serde(rename = "dge", alias = "Dodge")]
    Dodge,
    /// Enhancement bonus
    #[serde(rename = "en", alias = "enh", alias = "Enhancement")]
    Enhancement,
    /// Epic bonus
    ///
    /// This bonus is typically used in epic destinies where different trees may provide bonuses to
    /// a particular attribute, but only the highest will count
    #[serde(rename = "ep", alias = "epc", alias = "Epic")]
    Epic,
    /// Equipment Bonus
    #[serde(rename = "eq", alias = "Equipment")]
    Equipment,
    /// Exceptional bonus
    #[serde(rename = "ex", alias = "exc", alias = "Exceptional")]
    Exceptional,
    /// Feat bonus
    #[serde(rename = "fe", alias = "fea", alias = "Feat")]
    Feat,
    /// Festive bonus
    #[serde(rename = "fs", alias = "fst", alias = "Festive")]
    Festive,
    /// Fortune bonus
    #[serde(rename = "fr", alias = "for", alias = "Fortune")]
    Fortune,
    /// Guild
    #[serde(rename = "gu", alias = "gui", alias = "Guild")]
    Guild,
    /// Insightful bonus
    #[serde(rename = "in", alias = "ins", alias = "Insightful")]
    Insightful,
    /// Legendary bonus
    #[serde(rename = "lg", alias = "leg", alias = "Legendary")]
    Legendary,
    /// Luck Bonus
    #[serde(rename = "lk", alias = "lck", alias = "Luck")]
    Luck,
    /// Morale bonus
    #[serde(rename = "mr", alias = "mor", alias = "Morale")]
    Morale,
    /// Music bonus
    #[serde(rename = "mu", alias = "Music")]
    Music,
    /// Primal bonus
    #[serde(rename = "pm", alias = "pri", alias = "Primal")]
    Primal,
    /// Profane bonus
    #[serde(rename = "pf", alias = "pr", alias = "pro", alias = "Profane")]
    Profane,
    /// Quality bonus
    #[serde(rename = "qu", alias = "qual", alias = "Quality")]
    Quality,
    /// Racial bonus
    #[serde(rename = "ra", alias = "race", alias = "Racial")]
    Racial,
    /// Sacred bonus
    #[serde(rename = "sc", alias = "sac", alias = "Sacred")]
    Sacred,
    /// Size bonus
    #[serde(rename = "sz", alias = "sze", alias = "Size")]
    Size,
    /// Shield bonus
    #[serde(rename = "shi", alias = "Shield")]
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
    #[serde(rename = "s", alias = "stck", alias = "Stacking")]
    Stacking,
    /// Used when things don't stack, but they don't really have a bonus type.
    #[serde(rename = "st", alias = "std", alias = "Standard")]
    Standard,
    /// Spooky type
    #[serde(rename = "spo", alias = "Spooky")]
    Spooky,
}

impl Display for BonusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equipment => write!(f, "Equipment"),
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
            Self::Guild => write!(f, "Guild"),
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

impl BonusType {
    /// All different values
    pub const VALUES: [Self; 30] = [
        Self::AbilityModifier,
        Self::ActionBoost,
        Self::Alchemical,
        Self::Artifact,
        Self::Competence,
        Self::Deflection,
        Self::Equipment,
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
        Self::Guild,
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
    ];
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

impl StaticValues for BonusType {
    fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }
}
