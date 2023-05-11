use crate::logic::{bonus::{Bonus, BonusType}, attribute::Attribute};

use super::{Ability, ElementalType, Offensive, SpellPower, SpellSchool};

macro_rules! set_bonuses {
    ($($id: ident, $name: expr => ($($count: expr => $bonuses: expr),*)),*) => {
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
            pub fn get_bonuses(&self, value: f32) -> Option<Vec<$crate::logic::bonus::Bonus>> {
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
        $crate::logic::bonus::BonusSource::Attribute($crate::logic::attribute::Attribute::SetBonus(
            SetBonus::$set_bonus,
        ))
    };
}

set_bonuses!(
    LegendaryEldersKnowledge, "Legendary Elder's Knowledge" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Universal), BonusType::Artifact, 6f32, set_bonus_source!(LegendaryEldersKnowledge), None),
            Bonus::new(Attribute::SpellCriticalDamage(SpellPower::Universal), BonusType::Legendary, 15f32, set_bonus_source!(LegendaryEldersKnowledge), None),
        ]
    ),
    LegendaryVulkoorsChosen, "Legendary Vulkoor's Chosen" => (
        3f32 => vec![
            Bonus::new(Attribute::ElementalResistance(ElementalType::Poison), BonusType::Artifact, 30f32, set_bonus_source!(LegendaryVulkoorsChosen), None),
            Bonus::new(Attribute::Offensive(Offensive::SneakAttackDice), BonusType::Artifact, 3f32, set_bonus_source!(LegendaryVulkoorsChosen), None),
            Bonus::new(Attribute::SavingThrow(super::SavingThrow::All), BonusType::Artifact, 3f32, set_bonus_source!(LegendaryVulkoorsChosen), None),
            Bonus::new(Attribute::AbilityScore(super::Ability::Dexterity), BonusType::Artifact, 3f32, set_bonus_source!(LegendaryVulkoorsChosen), None),
            Bonus::new(Attribute::AbilityScore(super::Ability::Constitution), BonusType::Artifact, 3f32, set_bonus_source!(LegendaryVulkoorsChosen), None),
        ]
    ),
    ArcaneGuardian, "Arcane Guardian" => (
        3f32 => vec![
            Bonus::new(Attribute::MagicalSheltering, BonusType::Artifact, 30f32, set_bonus_source!(ArcaneGuardian), None)
        ]
    ),
    WildFortitude, "Wild Fortitude" => (
        3f32 => vec![
            Bonus::new(Attribute::AbilityScore(Ability::Constitution), BonusType::Artifact, 3f32, set_bonus_source!(WildFortitude), None)
        ]
    ),
    LegendaryHruitsInfluence, "Legendary Hruit's Influence" => (
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
);
