use std::fmt::Display;

use enum_map::Enum;

/// Describes the stacking-type of a bonus.
///
/// Bonuses with the same [`BonusType`] will not stack, meaning that only the highest value will be
/// added. However, bonuses of different [`BonusType`] will stack.
///
/// Any bonus with a type of [`BonusType::Stacking`] will always stack no matter what.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Enum)]
pub enum BonusType {
    // TODO: Documentation
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
    /// use builder_core::bonus::BonusType;
    /// let bonus_type: BonusType = Default::default();
    /// assert_eq!(BonusType::Stacking, bonus_type);
    /// ```
    Stacking,
    /// Spooky type
    Spooky,
}

impl Display for BonusType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BonusType::AbilityModifier => "Ability Modifier",
                BonusType::ActionBoost => "Action Boost",
                BonusType::Alchemical => "Alchemical",
                BonusType::Artifact => "Artifact",
                BonusType::Competence => "Competence",
                BonusType::Deflection => "Deflection",
                BonusType::Enhancement => "Enhancement",
                BonusType::Epic => "Epic",
                BonusType::Exceptional => "Exceptional",
                BonusType::Feat => "Feat",
                BonusType::Festive => "Festive",
                BonusType::Insightful => "Insightful",
                BonusType::Legendary => "Legendary",
                BonusType::Morale => "Morale",
                BonusType::Music => "Music",
                BonusType::Primal => "Primal",
                BonusType::Profane => "Profane",
                BonusType::Quality => "Quality",
                BonusType::Sacred => "Sacred",
                BonusType::Shield => "Shield",
                BonusType::Size => "Size",
                BonusType::Stacking => "Stacking",
                BonusType::Spooky => "Spooky",
            }
        )
    }
}

// impl ToString for BonusType {
//     fn to_string(&self) -> String {
//         match self {
//         }
//     }
// }

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
    /// use builder_core::bonus::BonusType;
    ///
    /// assert!(BonusType::Stacking.is_stacking());
    /// assert!(!BonusType::Enhancement.is_stacking());
    /// ```
    ///
    /// [`Stacking`]: BonusType::Stacking
    pub fn is_stacking(&self) -> bool {
        matches!(self, Self::Stacking)
    }
}
