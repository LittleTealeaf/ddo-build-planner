use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Describes the stacking-type of a bonus.
///
/// Bonuses with the same [`BonusType`] will not stack, meaning that only the highest value will be
/// added. However, bonuses of different [`BonusType`] will stack.
///
/// Any bonus with a type of [`BonusType::Stacking`] will always stack no matter what.
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BonusType {
    /// Bonuses that come from [`Attribute::AbilityModifier`]
    ///
    /// [`Attribute::AbilityModifier`]: crate::attribute::Attribute::AbilityModifier
    AbilityModifier,
    /// "Action Boost" bonuses
    ///
    /// These are typically bonuses that come from action boosts, or action boost-like buffs.
    ActionBoost,
    /// Alchemical bonuses
    Alchemical,
    /// Artifact bonuses
    ///
    /// These are typically bonuses from named item sets
    Artifact,
    /// Competence bonuses
    Competence,
    /// Deflection bonus
    Deflection,
    /// Dodge
    Dodge,
    /// Enhancement bonus
    Enhancement,
    /// Epic bonus
    ///
    /// This bonus is typically used in epic destinies where different trees may provide bonuses to
    /// a particular attribute, but only the highest will count
    Epic,
    /// Exceptional bonus
    Exceptional,
    /// Feat bonus
    Feat,
    /// Festive bonus
    Festive,
    /// Insightful bonus
    Insightful,
    /// Legendary bonus
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
    Profane,
    /// Quality bonus
    Quality,
    /// Racial bonus
    Racial,
    /// Sacred bonus
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
    Stacking,
    /// Used when things don't stack, but they don't really have a bonus type.
    Standard,
    /// Spooky type
    Spooky,
}

impl Display for BonusType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
