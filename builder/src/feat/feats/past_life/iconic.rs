use core::fmt::{self, Display};

use rust_decimal::prelude::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{BonusTemplate, BonusType, Condition, ConditionFold},
    types::{
        damage_type::DamageType,
        flag::{MainHandType, OffHandType},
        item_type::WeaponType,
        race::Race,
        saving_throw::SavingThrow,
        sheltering::Sheltering,
        spell_school::SpellSchool,
        tactics::Tactics,
        toggle::{GetToggleGroup, ToToggle, Toggle},
        toggle_group::ToggleGroup,
        weapon_attribute::{WeaponHand, WeaponStat},
    },
};

/// Iconic Past Life Feats
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IconicPastLife(Race);

impl IconicPastLife {
    /// Acceptable versions of this Iconic Past Life
    pub const RACES: [Self; 9] = [
        Self(Race::Scourge),
        Self(Race::Bladeforged),
        Self(Race::DeepGnome),
        Self(Race::PurpleDragonKnight),
        Self(Race::Razorclaw),
        Self(Race::Shadarkai),
        Self(Race::Morninglord),
        Self(Race::Trailblazer),
        Self(Race::Scoundrel),
    ];
}

impl Display for IconicPastLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(race) = self;
        write!(f, "{race} Past Life")
    }
}

impl StaticOptions for IconicPastLife {
    fn get_static() -> impl Iterator<Item = Self> {
        Self::RACES.into_iter()
    }
}

impl GetBonuses for IconicPastLife {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        if value <= Decimal::ZERO {
            return None;
        }

        let value = value.min(dec!(3));

        let Self(race) = self;

        match race {
            Race::Scourge => Some(vec![
                BonusTemplate::toggle(Self(Race::Scourge)),
                BonusTemplate::new(SavingThrow::Fortitude, BonusType::Stacking, 1),
                // TODO: Scourge: +2% doublestrike / life (stance)
            ]),
            Race::Bladeforged => Some(vec![
                BonusTemplate::toggle(Self(Race::Bladeforged)),
                // TODO: Fortification +5% / life (passive)
                BonusTemplate::new(
                    Attribute::SpellPower(DamageType::Repair.into()),
                    BonusType::Stacking,
                    value * dec!(10),
                )
                .with_condition(Condition::toggled(Self(Race::Bladeforged))),
            ]),
            Race::DeepGnome => Some(vec![
                BonusTemplate::toggle(Self(Race::DeepGnome)),
                BonusTemplate::new(Sheltering::Magical, BonusType::Stacking, value * dec!(3)),
                BonusTemplate::new(
                    Attribute::SpellDC(SpellSchool::Illusion.into()),
                    BonusType::Stacking,
                    value,
                )
                .with_condition(Condition::toggled(Self(Race::DeepGnome))),
                BonusTemplate::new(
                    Attribute::SpellPower(DamageType::Acid.into()),
                    BonusType::Stacking,
                    value * dec!(5),
                )
                .with_condition(Condition::toggled(Self(Race::DeepGnome))),
            ]),
            Race::PurpleDragonKnight => Some(vec![
                BonusTemplate::toggle(Self(Race::PurpleDragonKnight)),
                BonusTemplate::new(Sheltering::Physical, BonusType::Stacking, value * dec!(3)),
                BonusTemplate::new(SavingThrow::All, BonusType::ActionBoost, value * dec!(3))
                    .with_condition(Condition::toggled(Self(Race::PurpleDragonKnight))),
                BonusTemplate::new(
                    Attribute::MovementSpeed,
                    BonusType::ActionBoost,
                    value * dec!(10),
                )
                .with_condition(Condition::toggled(Self(Race::PurpleDragonKnight))),
            ]),
            Race::Razorclaw => {
                fn build_condition<F, A>(fun: F) -> Condition
                where
                    F: Fn(WeaponType) -> A,
                    A: Into<Attribute>,
                {
                    Condition::toggled(IconicPastLife(Race::Razorclaw))
                        & WeaponType::MELEE_WEAPONS
                            .map(|wt| Condition::has(fun(wt)))
                            .cond_any()
                            .unwrap_or(Condition::TRUE)
                }

                let main_hand = build_condition(MainHandType::Weapon);
                let off_hand = build_condition(OffHandType::Weapon);

                Some(vec![
                    BonusTemplate::toggle(Self(Race::Razorclaw)),
                    BonusTemplate::new(SavingThrow::Will, BonusType::Stacking, value),
                    BonusTemplate::new(
                        (WeaponHand::Main, WeaponStat::Attack),
                        BonusType::Stacking,
                        value,
                    )
                    .with_condition(main_hand.clone()),
                    BonusTemplate::new(
                        (WeaponHand::Main, WeaponStat::Damage),
                        BonusType::Stacking,
                        value,
                    )
                    .with_condition(main_hand),
                    BonusTemplate::new(
                        (WeaponHand::Off, WeaponStat::Attack),
                        BonusType::Stacking,
                        value,
                    )
                    .with_condition(off_hand.clone()),
                    BonusTemplate::new(
                        (WeaponHand::Off, WeaponStat::Damage),
                        BonusType::Stacking,
                        value,
                    )
                    .with_condition(off_hand),
                ])
            }
            Race::Shadarkai => Some(vec![
                BonusTemplate::toggle(Self(Race::Shadarkai)),
                // TODO: +1% dodge / past life passive
            ]),
            Race::Morninglord => Some(vec![
                BonusTemplate::toggle(Self(Race::Morninglord)),
                BonusTemplate::new(
                    Attribute::SpellPower(DamageType::Positive.into()),
                    BonusType::Stacking,
                    value * dec!(3),
                ),
                BonusTemplate::new(
                    Attribute::SpellPower(DamageType::Light.into()),
                    BonusType::Stacking,
                    value * Decimal::TEN,
                )
                .with_condition(Condition::toggled(Self(Race::Morninglord))),
                BonusTemplate::new(
                    Attribute::SpellPower(DamageType::Alignment.into()),
                    BonusType::Stacking,
                    value * Decimal::TEN,
                )
                .with_condition(Condition::toggled(Self(Race::Morninglord))),
            ]),
            Race::Trailblazer => Some(vec![
                BonusTemplate::toggle(Self(Race::Trailblazer)),
                BonusTemplate::new(SavingThrow::Traps, BonusType::Stacking, value),
                BonusTemplate::new(Tactics::Trip, BonusType::Stacking, value)
                    .with_condition(Condition::toggled(Self(Race::Trailblazer))),
            ]),
            Race::Scoundrel => Some(vec![
                BonusTemplate::toggle(Self(Race::Scoundrel)),
                BonusTemplate::new(SavingThrow::Reflex, BonusType::Stacking, value),
                BonusTemplate::new(
                    Attribute::MovementSpeed,
                    BonusType::Standard,
                    value * dec!(10),
                )
                .with_condition(Condition::toggled(Self(Race::Morninglord))),
            ]),
            _ => None,
        }
    }
}

impl ToToggle for IconicPastLife {
    fn to_toggle(self) -> Toggle {
        Toggle::IconicPastLife(self)
    }
}

impl GetToggleGroup for IconicPastLife {
    fn custom_toggle_group(&self) -> Option<ToggleGroup> {
        Some(ToggleGroup::IconicPastLife)
    }
}
