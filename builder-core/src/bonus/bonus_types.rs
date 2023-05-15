
/// Identifies the type that a [Bonus](crate::bonus::Bonus) is.
///
/// Implements the idea that Bonuses of the same type do not stack, and bonuses of different types stack.
///
/// The [`Stacking`] bonus type indicates that a bonus stacks with all other bonuses.
///
/// [`Stacking`]: BonusType::Stacking

#[derive(PartialEq, Eq, Copy, Clone, Hash, serde::Serialize, serde::Deserialize, Debug)]
pub enum BonusType {
    /// Describes bonuses from [Ability Modifiers](crate::attribute::Ability).
    ///
    /// This is generally used when it is possible that multiple [Abilities](crate::attribute::Ability) can be used for a certain [Attribute](crate::attribute::Attribute), but only the highest one should be added
    AbilityModifier,
    /// Describes bonuses with the Action Boost type.
    ActionBoost,
    /// Describes bonuses with the Alchemical type.
    Alchemical,
    /// Describes bonuses with the Artifact type.
    ///
    /// Often used with named item sets.
    Artifact,
    /// Describes bonuses with the Competence type.
    Competence,
    /// Describes bonuses with the Deflection type.
    Deflection,
    /// Describes bonuses with the Enhancement type.
    Enhancement,
    /// Describes bonuses from Epic sources.
    ///
    /// This is primarily used in instances, such as Epic Destinies, where bonuses can come from different trees and only the highest value is taken.
    Epic,
    /// Describes bonuses with the Exceptional type.
    Exceptional,
    /// Describes bonuses with the Feat type.
    Feat,
    /// Describes bonuses with the Festive type.
    Festive,
    /// Describes bonuses with the Insightful type.
    Insightful,
    /// Describes bonuses with the Legendary type.
    Legendary,
    /// Describes bonuses with the Morale type.
    Morale,
    /// Describes bonuses with the Music type.
    Music,
    /// Describes bonuses with the Primal type.
    Primal,
    /// Describes bonuses with the Profane type.
    Profane,
    /// Describes bonuses with the Quality type.
    Quality,
    /// Describes bonuses with the Sacred type.
    Sacred,
    /// Describes bonuses with the Size type.
    Size,
    /// Describes bonuses with the Shield type.
    Shield,
    /// Describes bonuses that stack with anything.
    ///
    /// This is the most used and therefore default bonus type, as it indicates that the bonus should be added no matter what. With the use of a well built source system, the need for various artificial bonus types is removed, so this is used in many circumstances.
    Stacking,
    /// Describes bonuses with the Spooky type.
    Spooky,
}

impl ToString for BonusType {
    fn to_string(&self) -> String {
        String::from(match self {
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
        })
    }
}

impl BonusType {
    /// Returns `true` if the bonus type is [`Stacking`].
    ///
    /// [`Stacking`]: BonusType::Stacking
    pub fn is_stacking(&self) -> bool {
        matches!(self, Self::Stacking)
    }
}

impl Default for BonusType {
    fn default() -> Self {
        Self::Stacking
    }
}
