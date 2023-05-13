use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusType},
};

use super::{Ability, ElementalType, Offensive, SpellPower, SpellSchool};

macro_rules! set_bonuses {
    ($($id: ident $name: expr => ($($count: expr => $bonuses: expr)*))*) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        pub enum SetBonus {
            $($id),*
        }

        impl ToString for SetBonus {
            fn to_string(&self) -> String {
                String::from(match self {
                    $(Self::$id => $name),*
                })
            }
        }

        impl SetBonus {
            pub fn get_bonuses(&self, value: f32) -> Option<Vec<$crate::bonus::Bonus>> {
                match self {
                    $(Self::$id => {
                        let mut vec = Vec::new();
                        $(
                            if value >= $count {
                                vec.append(&mut $bonuses);
                            }
                        )*
                        if vec.len() > 0 {Some(vec)} else {None}
                    }),*
                }
            }
        }
    }
}

macro_rules! set_bonus_source {
    ($set_bonus: ident) => {
        $crate::bonus::BonusSource::Attribute($crate::attribute::Attribute::SetBonus(
            SetBonus::$set_bonus,
        ))
    };
}

set_bonuses!(
    LegendaryEldersKnowledge "Legendary Elder's Knowledge" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Universal), BonusType::Artifact, 6f32, set_bonus_source!(LegendaryEldersKnowledge), None),
            Bonus::new(Attribute::SpellCriticalDamage(SpellPower::Universal), BonusType::Legendary, 15f32, set_bonus_source!(LegendaryEldersKnowledge), None),
        ]
    )
    LegendaryVulkoorsChosen "Legendary Vulkoor's Chosen" => (
        3f32 => vec![
            Bonus::new(Attribute::ElementalResistance(ElementalType::Poison), BonusType::Artifact, 30f32, set_bonus_source!(LegendaryVulkoorsChosen), None),
            Bonus::new(Attribute::Offensive(Offensive::SneakAttackDice), BonusType::Artifact, 3f32, set_bonus_source!(LegendaryVulkoorsChosen), None),
            Bonus::new(Attribute::SavingThrow(super::SavingThrow::All), BonusType::Artifact, 3f32, set_bonus_source!(LegendaryVulkoorsChosen), None),
            Bonus::new(Attribute::AbilityScore(super::Ability::Dexterity), BonusType::Artifact, 3f32, set_bonus_source!(LegendaryVulkoorsChosen), None),
            Bonus::new(Attribute::AbilityScore(super::Ability::Constitution), BonusType::Artifact, 3f32, set_bonus_source!(LegendaryVulkoorsChosen), None),
        ]
    )
    ArcaneGuardian "Arcane Guardian" => (
        3f32 => vec![
            Bonus::new(Attribute::Defensive(super::Defensive::MagicalSheltering), BonusType::Artifact, 30f32, set_bonus_source!(ArcaneGuardian), None)
        ]
    )
    WildFortitude "Wild Fortitude" => (
        3f32 => vec![
            Bonus::new(Attribute::AbilityScore(Ability::Constitution), BonusType::Artifact, 3f32, set_bonus_source!(WildFortitude), None)
        ]
    )
    TouchOfPower "Touch of Power" => (
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Artifact, 25f32, set_bonus_source!(TouchOfPower), None)
        ]
    )
    LegendaryHruitsInfluence "Legendary Hruit's Influence" => (
        3f32 => vec![
            Bonus::new(Attribute::AbilityScore(Ability::Wisdom), BonusType::Artifact, 3f32, set_bonus_source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Fire), BonusType::Artifact, 30f32, set_bonus_source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Cold), BonusType::Artifact, 30f32, set_bonus_source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Electric), BonusType::Artifact, 30f32, set_bonus_source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Positive), BonusType::Artifact, 30f32, set_bonus_source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Fire), BonusType::Artifact, 6f32, set_bonus_source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Cold), BonusType::Artifact, 6f32, set_bonus_source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Electric), BonusType::Artifact, 6f32, set_bonus_source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Positive), BonusType::Artifact, 6f32, set_bonus_source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellFocus(SpellSchool::All), BonusType::Artifact, 3f32, set_bonus_source!(LegendaryHruitsInfluence), None),

        ]
    )
    LegendaryDreadIsleCurse "The Legendary Dread Isle's Curse" => (
        5f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::MeleePower), BonusType::Profane, 15f32, set_bonus_source!(LegendaryDreadIsleCurse), None),
            Bonus::new(Attribute::Offensive(Offensive::RangedPower), BonusType::Profane, 15f32, set_bonus_source!(LegendaryDreadIsleCurse), None),
            Bonus::new(Attribute::Defensive(super::Defensive::PhysicalSheltering), BonusType::Profane, 30f32, set_bonus_source!(LegendaryDreadIsleCurse), None),
            Bonus::new(Attribute::SpellFocus(SpellSchool::All), BonusType::Profane, 2f32, set_bonus_source!(LegendaryDreadIsleCurse), None),
            Bonus::new(Attribute::AbilityScore(Ability::All), BonusType::Profane, 2f32, set_bonus_source!(LegendaryDreadIsleCurse), None),
            Bonus::new(Attribute::WeaponStat(super::WeaponHand::Both, super::WeaponStat::Attack), BonusType::Profane, 3f32, set_bonus_source!(LegendaryDreadIsleCurse), None),
            Bonus::new(Attribute::WeaponStat(super::WeaponHand::Both, super::WeaponStat::Damage), BonusType::Profane, 3f32, set_bonus_source!(LegendaryDreadIsleCurse), None),

        ]
    )
);
