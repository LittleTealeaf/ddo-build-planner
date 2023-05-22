use crate::{
    attribute::{Attribute, GetBonuses, GetCloned},
    bonus::{Bonus, BonusType, Condition},
};


use super::{Flag, WeaponHand, WeaponStat};

/// Describes the six main stats for a character.
#[derive(
    Clone, Copy, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug, enum_map::Enum,
)]
pub enum Ability {
    /// Describes how strong the person is.
    Strength,
    /// Describes how flexible the person is
    Dexterity,
    /// Describes how much the person can take a hit.
    Constitution,
    /// Describes how smart the person is
    Intelligence,
    /// Describes how wise the person is
    Wisdom,
    /// Describes how influential the person is
    Charisma,
    /// References all abilities at once.
    ///
    /// This is used mostly when giving some bonus to all ability scores, as it clones to the others using [`Self::get_cloned()`]
    All,
}

impl ToString for Ability {
    fn to_string(&self) -> String {
        String::from(match self {
            Ability::Strength => "Strength",
            Ability::Dexterity => "Dexterity",
            Ability::Constitution => "Constitution",
            Ability::Intelligence => "Intelligence",
            Ability::Wisdom => "Wisdom",
            Ability::Charisma => "Charisma",
            Ability::All => "All Abilities",
        })
    }
}

impl Ability {
    /// All ability values except for [`Ability::All`]
    pub const VALUES: [Ability; 6] = [Ability::Strength, Ability::Dexterity, Ability::Constitution, Ability::Intelligence, Ability::Wisdom, Ability::Charisma];

    /// Converts an ability to the [`Attribute::AbilityModifier`] value
    #[inline(always)]
    pub fn into_modifier_attribute(self) -> Attribute {
        Attribute::AbilityModifier(self)
    }
}

impl From<Ability> for Attribute {
    #[inline(always)]
    fn from(value: Ability) -> Attribute {
        Attribute::Ability(value)
    }
}

/// Dummy Struct to differentiate bonuses for [Ability]
pub struct _AbilityScore;
/// Dummy Struct to differentiate bonuses for [Ability]
pub struct _AbilityModifier;

macro_rules! modifier_skill {
    ($modifier: ident, $skill: ident, $value: expr) => {
        Bonus::new(
            $crate::attribute::Attribute::Skill($crate::attribute::Skill::$skill),
            $crate::bonus::BonusType::AbilityModifier,
            $value,
            $crate::bonus::BonusSource::Attribute($crate::attribute::Attribute::AbilityModifier(
                Ability::$modifier,
            )),
            None,
        )
    };
}

macro_rules! modifier_saving_throw {
    ($modifier: ident, $saving_throw: ident, $value: expr, $def: expr) => {
        Bonus::new(
            $crate::attribute::Attribute::SavingThrow(
                $crate::attribute::SavingThrow::$saving_throw,
            ),
            $crate::bonus::BonusType::AbilityModifier,
            $value,
            $crate::bonus::BonusSource::Attribute($crate::attribute::Attribute::AbilityModifier(
                Ability::$modifier,
            )),
            if $def {
                None
            } else {
                Some(vec![$crate::bonus::Condition::Has(
                    $crate::attribute::Attribute::Flag(
                        $crate::attribute::Flag::AbilityToSavingThrow(
                            Ability::$modifier,
                            $crate::attribute::SavingThrow::$saving_throw,
                        ),
                    ),
                )])
            },
        )
    };
}

impl GetBonuses<_AbilityScore> for Ability {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        if let Ability::All = self {
            None
        } else {
            Some(vec![Bonus::new(
                Attribute::AbilityModifier(*self),
                BonusType::Stacking,
                ((value - 10f32) / 2f32).floor(),
                Attribute::to_source(*self),
                None,
            )])
        }
    }
}

impl GetBonuses<_AbilityModifier> for Ability {
    /// Returns a list of modifier bonuses when provided the current modifier value
    ///
    /// The goal of this function is to link each ability to other attirbutes. This includes, but is not limited to, skills, saving throws, and attack/damage modifiers.
    ///
    /// This function returns an [Option] because of the case of [`Self::All`], where there should not be any modifier bonuses. Therefore, if you try to get the modifier bonuses of [`Self::All`], you will get [None]
    ///
    /// ```
    /// use builder_core::attribute::{GetBonuses, sub::{Ability, _AbilityModifier}};
    ///
    /// assert_eq!(None, GetBonuses::<_AbilityModifier>::get_bonuses(&Ability::All, 32f32));
    /// ```
    ///
    /// However, for each other ability, a list of bonuses will be returned that should be added to the [Breakdowns](crate::breakdown::Breakdowns). Note that this function is used by [Breakdowns](crate::breakdown::Breakdowns) and should not be manually used.
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        let mut values = vec![];

        values.append(&mut match self {
            Self::Strength => Some(vec![
                modifier_skill!(Strength, Jump, value),
                modifier_skill!(Strength, Swim, value),
                modifier_saving_throw!(Strength, Reflex, value, false),
                modifier_saving_throw!(Strength, Fortitude, value, false),
                modifier_saving_throw!(Strength, Will, value, false),
            ]),
            Self::Dexterity => Some(vec![
                modifier_skill!(Dexterity, Balance, value),
                modifier_skill!(Dexterity, Hide, value),
                modifier_skill!(Dexterity, MoveSilently, value),
                modifier_skill!(Dexterity, OpenLock, value),
                modifier_skill!(Dexterity, Tumble, value),
                modifier_saving_throw!(Dexterity, Reflex, value, true),
                modifier_saving_throw!(Dexterity, Fortitude, value, false),
                modifier_saving_throw!(Dexterity, Will, value, false),
            ]),
            Self::Constitution => Some(vec![
                modifier_skill!(Constitution, Concentration, value),
                modifier_saving_throw!(Constitution, Reflex, value, false),
                modifier_saving_throw!(Constitution, Fortitude, value, true),
                modifier_saving_throw!(Constitution, Will, value, false),
            ]),
            Self::Intelligence => Some(vec![
                modifier_skill!(Intelligence, DisableDevice, value),
                modifier_skill!(Intelligence, Repair, value),
                modifier_skill!(Intelligence, Search, value),
                modifier_skill!(Intelligence, SpellCraft, value),
                modifier_saving_throw!(Intelligence, Reflex, value, false),
                modifier_saving_throw!(Intelligence, Fortitude, value, false),
                modifier_saving_throw!(Intelligence, Will, value, false),
            ]),
            Self::Wisdom => Some(vec![
                modifier_skill!(Wisdom, Heal, value),
                modifier_skill!(Wisdom, Listen, value),
                modifier_skill!(Wisdom, Spot, value),
                modifier_saving_throw!(Wisdom, Reflex, value, false),
                modifier_saving_throw!(Wisdom, Fortitude, value, false),
                modifier_saving_throw!(Wisdom, Will, value, true),
            ]),
            Self::Charisma => Some(vec![
                modifier_skill!(Charisma, Bluff, value),
                modifier_skill!(Charisma, Diplomacy, value),
                modifier_skill!(Charisma, Haggle, value),
                modifier_skill!(Charisma, Intimidate, value),
                modifier_skill!(Charisma, Perform, value),
                modifier_skill!(Charisma, UseMagicalDevice, value),
                modifier_saving_throw!(Charisma, Reflex, value, false),
                modifier_saving_throw!(Charisma, Fortitude, value, false),
                modifier_saving_throw!(Charisma, Will, value, false),
            ]),
            Self::All => None,
        }?);

        values.append(&mut vec![
            Bonus::new(
                (WeaponHand::Main, WeaponStat::Attack()).into(),
                BonusType::AbilityModifier,
                value,
                self.into_modifier_attribute().into(),
                Some(vec![Condition::Has(
                    Flag::AbilityToAttack(*self, super::WeaponHand::Main).into(),
                )]),
            ),
            Bonus::new(
                (WeaponHand::Off, WeaponStat::Attack()).into(),
                BonusType::AbilityModifier,
                value,
                self.into_modifier_attribute().into(),
                Some(vec![Condition::Has(
                    Flag::AbilityToAttack(*self, super::WeaponHand::Off).into(),
                )]),
            ),
            Bonus::new(
                (WeaponHand::Main, WeaponStat::Damage()).into(),
                BonusType::AbilityModifier,
                value,
                self.into_modifier_attribute().into(),
                Some(vec![Condition::Has(
                    Flag::AbilityToDamage(*self, super::WeaponHand::Main).into(),
                )]),
            ),
            Bonus::new(
                (WeaponHand::Off, WeaponStat::Damage()).into(),
                BonusType::AbilityModifier,
                value,
                self.into_modifier_attribute().into(),
                Some(vec![Condition::Has(
                    Flag::AbilityToDamage(*self, super::WeaponHand::Off).into(),
                )]),
            ),
        ]);

        Some(values)
    }
}

impl GetCloned<Ability> for Ability {
    fn get_cloned(&self) -> Option<Vec<Ability>> {
        if let Self::All = self {
            Some(Vec::from(Ability::VALUES))
        } else {
            None
        }
    }
}
