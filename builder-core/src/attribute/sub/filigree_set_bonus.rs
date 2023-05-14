use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusType},
};

use super::{
    Ability, ElementalType, HealAmp, HitPoint, SavingThrow, Skill, SpellPoint, SpellPower,
    SpellSchool, Tactics, WeaponHand, WeaponStat,
};

macro_rules! filigree_set_bonuses {
    ($value: ident, $($set_name: ident $set_string: expr => ($($count: expr => $bonuses: expr)*))*) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Debug)]
        pub enum FiligreeSet {
            $($set_name),*
        }

        impl ToString for FiligreeSet {
            fn to_string(&self) -> String {
                String::from(match self {
                    $(Self::$set_name => $set_string),*
                })
            }
        }

        impl FiligreeSet {
            pub fn get_attribute_bonuses(&self, $value: f32) -> Option<Vec<$crate::bonus::Bonus>> {
                let mut bonuses = Vec::new();

                match self {
                    $(Self::$set_name => {
                        $(if $value >= $count {
                            bonuses.append(&mut $bonuses);
                        })*
                    }),*
                }

                if bonuses.len() > 0 {
                    Some(bonuses)
                } else {
                    None
                }
            }
        }

        #[cfg(test)]
        mod gen_tests {
            use super::*;

            #[test]
            fn attributes_have_correct_source() {
                $(
                    $(
                        {
                            let $value = $count;
                            let bonuses: Vec<$crate::bonus::Bonus> = $bonuses;
                            for bonus in bonuses {
                                assert_eq!(
                                    source!($set_name),
                                    bonus.get_source(),
                                    "A [{}] bonus has a [{}] source",
                                    FiligreeSet::$set_name.to_string(),
                                    bonus.get_source().to_string()
                                );
                            }
                        }
                    )*
                )*
            }
        }
    };
}

macro_rules! source {
    ($set: ident) => {
        $crate::bonus::BonusSource::Attribute($crate::attribute::Attribute::FiligreeSet(
            FiligreeSet::$set,
        ))
    };
}

filigree_set_bonuses!(
    value,
    TheBeastsMantle "The Beast's Mantle" => (
        2f32 => vec![
            Bonus::new(Attribute::NaturalArmor(), BonusType::Stacking, 10f32, source!(TheBeastsMantle), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::FortificationBypass(), BonusType::Stacking, 10f32, source!(TheBeastsMantle), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::ImbueDice(), BonusType::Stacking, 2f32, source!(TheBeastsMantle), None)
        ]
    )
    TheBloodFeast "The Blood Feast" => (
        2f32 => vec![
            Bonus::new(Attribute::Ability(Ability::Constitution), BonusType::Stacking, 1f32, source!(TheBloodFeast), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::PhysicalSheltering(), BonusType::Stacking, 5f32, source!(TheBloodFeast), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::MeleePower(), BonusType::Stacking, 5f32, source!(TheBloodFeast), None)
        ]
        5f32 => vec![
            // TODO: The Blood Feast Proc
        ]
    )
    Clerity "Clerity" => (
        2f32 => vec![
            Bonus::new(Attribute::Doublestrike(), BonusType::Stacking, 2f32, source!(Clerity), None),
            Bonus::new(Attribute::Doubleshot(), BonusType::Stacking, 2f32, source!(Clerity), None),
        ]
        3f32 => vec![
            // TODO: Permenant Haste Spell
        ]
    )
    // TODO: City's Beacon
    // TODO: The Cry of Battle
    DeadlyRain "Deadly Rain" => (
        2f32 => vec![
            Bonus::new(Attribute::Doubleshot(), BonusType::Stacking, 2f32, source!(DeadlyRain), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::WeaponStat(WeaponHand::Both, WeaponStat::Damage()), BonusType::Stacking, 2f32, source!(DeadlyRain), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::MissileDeflection(), BonusType::Stacking, 3f32, source!(DeadlyRain), None),
        ]
        5f32 => vec![
            // TODO: Action Boost bonus to Ranged Power
        ]
    )
    Electrocution "Electrocution" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Electric), BonusType::Stacking, 50f32, source!(Electrocution), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::ImbueDice(), BonusType::Stacking, 2f32, source!(Electrocution), None),
        ]
        4f32 => vec![
            Bonus::new(Attribute::ElementalAbsorption(ElementalType::Electric), BonusType::Shield, 50f32, source!(Electrocution), None)
        ]
    )
    EmbracedByLight "Embraced by Light" => (
        2f32 => vec![
            Bonus::new(Attribute::HealAmp(HealAmp::Positive), BonusType::Stacking, 5f32, source!(EmbracedByLight), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Stacking, 10f32, source!(EmbracedByLight), None),
        ]
        4f32 => vec![
            Bonus::new(Attribute::SavingThrow(SavingThrow::All), BonusType::Stacking, 1f32, source!(EmbracedByLight), None),
        ]
        5f32 => vec![
            // TODO: Casting level with spells / items
        ]
    )
    EnlightenedStep "Enlightened Step" => (
        2f32 => vec![
            Bonus::new(Attribute::HealAmp(HealAmp::Positive), BonusType::Stacking, 5f32, source!(EnlightenedStep), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::MeleePower(), BonusType::Stacking, 5f32, source!(EnlightenedStep), None),
            Bonus::new(Attribute::RangedPower(), BonusType::Stacking, 5f32, source!(EnlightenedStep), None),
        ]
        4f32 => vec![
            Bonus::new(Attribute::PhysicalSheltering(), BonusType::Stacking, 5f32, source!(EnlightenedStep), None)
        ]
        5f32 => vec![
            Bonus::new(Attribute::Doubleshot(), BonusType::Stacking, 5f32, source!(EnlightenedStep), None),
            Bonus::new(Attribute::Doublestrike(), BonusType::Stacking, 5f32, source!(EnlightenedStep), None),
        ]
    )
    EyeOfTheBeholder "Eye of the Beholder" => (
        2f32 => vec![
            Bonus::new(Attribute::Skill(Skill::Concentration), BonusType::Stacking, 4f32, source!(EyeOfTheBeholder), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellPenetration(), BonusType::Stacking, 1f32, source!(EyeOfTheBeholder), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::SpellFocus(SpellSchool::All), BonusType::Stacking, 2f32, source!(EyeOfTheBeholder), None)
        ]
    )
    FrozenWanderer "Frozen Wanderer" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Cold), BonusType::Stacking, 5f32, source!(FrozenWanderer), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::MagicalShelteringCap(), BonusType::Stacking, 10f32, source!(FrozenWanderer), None),
        ]
        4f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Cold), BonusType::Stacking, 50f32, source!(FrozenWanderer), None),
            Bonus::new(Attribute::ImbueDice(), BonusType::Stacking, 1f32, source!(FrozenWanderer), None),
        ]
    )
    GrandfathersShield "Grandfather's Shield" => (
        2f32 => vec![
            Bonus::new(Attribute::PhysicalSheltering(), BonusType::Stacking, 5f32, source!(GrandfathersShield), None),
        ]
        3f32 => vec![
            // TODO: +5% Armor Class
        ]
        4f32 => vec![
            Bonus::new(Attribute::SavingThrow(SavingThrow::Will), BonusType::Stacking, 2f32, source!(GrandfathersShield), None),
        ]
    )
    TheInevitableGrave "The Inevitable Grave" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::All), BonusType::Stacking, 10f32, source!(TheInevitableGrave), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::SavingThrow(SavingThrow::Will), BonusType::Stacking, 2f32, source!(TheInevitableGrave), None),
        ]
        4f32 => vec![
            Bonus::new(Attribute::SpellFocus(SpellSchool::Necromancy), BonusType::Stacking, 2f32, source!(TheInevitableGrave), None)
        ]
    )
    TheLongShadow "The Long Shadow" => (
        2f32 => vec![
            Bonus::new(Attribute::MeleePower(), BonusType::Stacking, 5f32, source!(TheLongShadow), None),
            Bonus::new(Attribute::RangedPower(), BonusType::Stacking, 5f32, source!(TheLongShadow), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::ImbueDice(), BonusType::Stacking, 2f32, source!(TheLongShadow), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::Dodge(), BonusType::Stacking, 1f32, source!(TheLongShadow), None),
            Bonus::new(Attribute::MaxDodge(), BonusType::Stacking, 1f32, source!(TheLongShadow), None),
        ]
        5f32 => vec![
            Bonus::new(Attribute::Tactics(Tactics::Assassinate), BonusType::Stacking, 2f32, source!(TheLongShadow), None)
        ]
    )
    MelonysMelody "Melody's Melony" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPoints(SpellPoint::Bonus), BonusType::Stacking, 50f32, source!(MelonysMelody), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::Ability(Ability::Charisma), BonusType::Stacking, 1f32, source!(MelonysMelody), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::SpellFocus(SpellSchool::Enchantment), BonusType::Stacking, 2f32, source!(MelonysMelody), None)
        ]
    )
    NystulsMysticalDefense "Nystul's Mystical Defense" => (
        2f32 => vec![
            Bonus::new(Attribute::MagicalSheltering(), BonusType::Stacking, 5f32, source!(NystulsMysticalDefense), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::SavingThrow(SavingThrow::All), BonusType::Stacking, 1f32, source!(NystulsMysticalDefense), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::HitPoints(HitPoint::Bonus), BonusType::Stacking, 100f32, source!(NystulsMysticalDefense), None)
        ]
        5f32 => vec![
            Bonus::new(Attribute::MagicalShelteringCap(), BonusType::Stacking, 40f32, source!(NystulsMysticalDefense), None)
        ]
    )
    OneAgainstMany "One Againt Many" => (
        2f32 => vec![
            Bonus::new(Attribute::MeleePower(), BonusType::Stacking, 5f32, source!(OneAgainstMany), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::Strikethrough(), BonusType::Stacking, 5f32, source!(OneAgainstMany), None)
        ]
        4f32 => vec![
            // TODO: One Against Many Proc
        ]
    )
    OttosIrrevocablePower "Otto's Irrevocable Power" => (
        2f32 => vec![
            Bonus::new(Attribute::MagicalSheltering(), BonusType::Stacking, 5f32, source!(OttosIrrevocablePower), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellPoints(SpellPoint::Bonus), BonusType::Stacking, 200f32, source!(OttosIrrevocablePower), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::SpellFocus(SpellSchool::All), BonusType::Stacking, 2f32, source!(OttosIrrevocablePower), None)
        ]
        5f32 => vec![
            // TODO: Wellspring / Arcane Insight bonus
        ]
    )
    Prowess "Prowess" => (
        2f32 => vec![
            Bonus::new(Attribute::PhysicalSheltering(), BonusType::Stacking, 5f32, source!(Prowess), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::MeleePower(), BonusType::Stacking, 5f32, source!(Prowess), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::BonusActionBoosts(), BonusType::Stacking, 2f32, source!(Prowess), None)
        ]
        5f32 => vec![
            // TODO: Prowess Procs
        ]
    )
    Purity "Purity" => (
        2f32 => vec![
            Bonus::new(Attribute::HealAmp(HealAmp::Positive), BonusType::Stacking, 5f32, source!(Purity), None)
        ]
        3f32 => vec![
            // TODO: Immunity to Mummy Rot and Natural Diseases
        ]
        4f32 => vec![
            // TODO: Immunity to Energy Drain
        ]
    )
    SnakeBite "Snake Bite" => (
        2f32 => vec![
            Bonus::new(Attribute::SavingThrow(SavingThrow::Poison), BonusType::Stacking, 4f32, source!(SnakeBite), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::ImbueDice(), BonusType::Stacking, 2f32, source!(SnakeBite), None)
        ]
    )
    SpinesOfTheManticore "Spines of the Manticore" => (
        2f32 => vec![
            Bonus::new(Attribute::WeaponStat(WeaponHand::Both, WeaponStat::Damage()), BonusType::Stacking, 2f32, source!(SpinesOfTheManticore), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::MissileDeflection(), BonusType::Stacking, 3f32, source!(SpinesOfTheManticore), None),
        ]
        4f32 => vec![
            Bonus::new(Attribute::RangedPower(), BonusType::Stacking, 10f32, source!(SpinesOfTheManticore), None)
        ]
    )
    SuckerPunch "Sucker Punch" => (
        2f32 => vec![
            Bonus::new(Attribute::MeleePower(), BonusType::Stacking, 10f32, source!(SuckerPunch), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::SneakAttackDice(), BonusType::Stacking, 1f32, source!(SuckerPunch), None)
        ]
    )
    ToHellAndBack "To Hell and Back" => (
        2f32 => vec![
            Bonus::new(Attribute::PhysicalSheltering(), BonusType::Stacking, 5f32, source!(ToHellAndBack), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::MagicalSheltering(), BonusType::Stacking, 10f32, source!(ToHellAndBack), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::SavingThrow(SavingThrow::All), BonusType::Stacking, 1f32, source!(ToHellAndBack), None)
        ]
    )
    TouchOfGrace "Touch of Grace" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPoints(SpellPoint::Bonus), BonusType::Stacking, 50f32, source!(TouchOfGrace), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Positive), BonusType::Stacking, 20f32, source!(TouchOfGrace), None)
        ]
    )
    TrappersDelight "Trapper's Delight" => (
        2f32 => vec![
            Bonus::new(Attribute::Skill(Skill::OpenLock), BonusType::Stacking, 1f32, source!(TrappersDelight), None),
            Bonus::new(Attribute::Skill(Skill::DisableDevice), BonusType::Stacking, 1f32, source!(TrappersDelight), None),
            Bonus::new(Attribute::Skill(Skill::Search), BonusType::Stacking, 1f32, source!(TrappersDelight), None),
            Bonus::new(Attribute::Skill(Skill::Spot), BonusType::Stacking, 1f32, source!(TrappersDelight), None),
            Bonus::new(Attribute::SavingThrow(SavingThrow::Traps), BonusType::Stacking, 1f32, source!(TrappersDelight), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::Skill(Skill::OpenLock), BonusType::Stacking, 2f32, source!(TrappersDelight), None),
            Bonus::new(Attribute::Skill(Skill::DisableDevice), BonusType::Stacking, 2f32, source!(TrappersDelight), None),
            Bonus::new(Attribute::Skill(Skill::Search), BonusType::Stacking, 2f32, source!(TrappersDelight), None),
            Bonus::new(Attribute::Skill(Skill::Spot), BonusType::Stacking, 2f32, source!(TrappersDelight), None),
            Bonus::new(Attribute::SavingThrow(SavingThrow::Traps), BonusType::Stacking, 2f32, source!(TrappersDelight), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::Skill(Skill::OpenLock), BonusType::Stacking, 3f32, source!(TrappersDelight), None),
            Bonus::new(Attribute::Skill(Skill::DisableDevice), BonusType::Stacking, 3f32, source!(TrappersDelight), None),
            Bonus::new(Attribute::Skill(Skill::Search), BonusType::Stacking, 3f32, source!(TrappersDelight), None),
            Bonus::new(Attribute::Skill(Skill::Spot), BonusType::Stacking, 3f32, source!(TrappersDelight), None),
            Bonus::new(Attribute::SavingThrow(SavingThrow::Traps), BonusType::Stacking, 3f32, source!(TrappersDelight), None)
        ]
    )
    Treachery "Treachery" => (
        2f32 => vec![
            Bonus::new(Attribute::MeleePower(), BonusType::Stacking, 10f32, source!(Treachery), None),
            Bonus::new(Attribute::RangedPower(), BonusType::Stacking, 10f32, source!(Treachery), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::SneakAttackDice(), BonusType::Stacking, 1f32, source!(Treachery), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::Tactics(Tactics::Assassinate), BonusType::Stacking, 2f32, source!(Treachery), None)
        ]
    )
    TwilightsCloak "Twilight's Cloak" => (
        2f32 => vec![
            Bonus::new(Attribute::SavingThrow(SavingThrow::Reflex), BonusType::Stacking, 2f32, source!(TwilightsCloak), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::MagicalSheltering(), BonusType::Stacking, 5f32, source!(TwilightsCloak), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::Dodge(), BonusType::Stacking, 1f32, source!(TwilightsCloak), None),
            Bonus::new(Attribute::MaxDodge(), BonusType::Stacking, 1f32, source!(TwilightsCloak), None),
        ]
    )
    Vigilance "Vigilance" => (
        2f32 => vec![
            Bonus::new(Attribute::Fortification(), BonusType::Stacking, 50f32, source!(Vigilance), None)
        ]
        3f32 => vec![
            // TODO: True Seeing
        ]
    )
    TheWreathOfFlame "The Wreath of Flame" => (
        2f32 => vec![
            Bonus::new(Attribute::ElementalAbsorption(ElementalType::Fire), BonusType::Stacking, 5f32, source!(TheWreathOfFlame), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::ElementalAbsorption(ElementalType::Fire), BonusType::Shield, 50f32, source!(TheWreathOfFlame), None),
        ]
        4f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Fire), BonusType::Stacking, 50f32, source!(TheWreathOfFlame), None),
            Bonus::new(Attribute::ImbueDice(), BonusType::Stacking, 1f32, source!(TheWreathOfFlame), None),
        ]
    )
    Zephyr "Zephyr" => (
        2f32 => vec![
            Bonus::new(Attribute::MagicalSheltering(), BonusType::Stacking, 5f32, source!(Zephyr), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::Dodge(), BonusType::Stacking, 1f32, source!(Zephyr), None),
            Bonus::new(Attribute::MaxDodge(), BonusType::Stacking, 1f32, source!(Zephyr), None),
        ]
        4f32 => vec![
            // TODO: Immunity to Slippery Surfaces
            // TODO: Immunity to Knockdown
        ]
    )
);
