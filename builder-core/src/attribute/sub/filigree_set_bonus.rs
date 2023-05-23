use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusType, Condition},
};

use super::{
    Ability, DifficultyCheck, ElementalType, Flag, HealAmp, HitPoint, Immunity, SavingThrow, Skill,
    SpellPoint, SpellPower, SpellSchool, SpellType, Tactics, ThreatType, Toggle, WeaponHand,
    WeaponStat,
};

macro_rules! filigree_set_bonuses {
    ($value: ident, $($set_name: ident $set_string: expr => ($($count: expr => $bonuses: expr)*))*) => {
        /// Represents the number of filigrees a player has towards each FiligreeSet.
        ///
        /// Fetching bonuses from this set will return a list of bonuses based on how many items the user is wearing of the set.
        #[derive(Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Debug, enum_map::Enum)]
        pub enum FiligreeSet {
            $(
                #[doc = $set_string]
                #[doc = " Filigree Set"]
                $set_name
            ),*
        }

        impl ToString for FiligreeSet {
            fn to_string(&self) -> String {
                String::from(match self {
                    $(Self::$set_name => $set_string),*
                })
            }
        }

        impl $crate::attribute::GetBonuses for FiligreeSet {
            fn get_bonuses(&self, $value: f32) -> Option<Vec<$crate::bonus::Bonus>> {
                let mut bonuses = Vec::new();

                match self {
                    $(Self::$set_name => {
                        $(if $value >= $count {
                            bonuses.append(&mut $bonuses);
                        })*
                    }),*
                }

                (bonuses.len() > 0).then(|| bonuses)
            }
        }

        impl From<FiligreeSet> for Attribute {
            #[inline(always)]
            fn from(value: FiligreeSet) -> Self {
                Attribute::FiligreeSet(value)
            }
        }

        #[cfg(test)]
        mod gen_tests {
            use super::*;

            #[test]
            fn bonuses_have_correct_source() {
                $(
                    $(
                        {
                            let $value = $count;
                            let bonuses: Vec<$crate::bonus::Bonus> = $bonuses;
                            for bonus in bonuses {
                                assert_eq!(
                                    source!($set_name),
                                    bonus.get_source(),
                                    "The [{}] bonus for [{}] has an incorrect source: [{}]",
                                    bonus.to_string(),
                                    FiligreeSet::$set_name.to_string(),
                                    bonus.get_source().to_string()
                                );
                            }
                        }
                    )*
                )*
            }

            #[test]
            fn zero_value_returns_no_bonuses() {
                $(
                    assert_eq!(None, $crate::attribute::GetBonuses::get_bonuses(&FiligreeSet::$set_name, 0f32));
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
            Bonus::new(Ability::Constitution.into(), BonusType::Stacking, 1f32, source!(TheBloodFeast), None)
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
            Bonus::new((WeaponHand::Both, WeaponStat::Damage).into(), BonusType::Stacking, 2f32, source!(DeadlyRain), None)
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
            Bonus::new(SpellPower::Electric.into_spell_power(), BonusType::Stacking, 50f32, source!(Electrocution), None),
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
            Bonus::new(HealAmp::Positive.into(), BonusType::Stacking, 5f32, source!(EmbracedByLight), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Stacking, 10f32, source!(EmbracedByLight), None),
        ]
        4f32 => vec![
            Bonus::new(SavingThrow::All.into(), BonusType::Stacking, 1f32, source!(EmbracedByLight), None),
        ]
        5f32 => vec![
            Bonus::new(Attribute::CasterLevel(SpellPower::Positive.into()), BonusType::Stacking, 2f32, source!(EmbracedByLight), None)
        ]
    )
    EnlightenedStep "Enlightened Step" => (
        2f32 => vec![
            Bonus::new(HealAmp::Positive.into(), BonusType::Stacking, 5f32, source!(EnlightenedStep), None),
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
            Bonus::new(Skill::Concentration.into(), BonusType::Stacking, 4f32, source!(EyeOfTheBeholder), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellPenetration(), BonusType::Stacking, 1f32, source!(EyeOfTheBeholder), None)
        ]
        4f32 => vec![
            Bonus::new(DifficultyCheck::Spell(SpellSchool::All.into()).into(), BonusType::Stacking, 2f32, source!(EyeOfTheBeholder), None)
        ]
    )
    FrozenWanderer "Frozen Wanderer" => (
        2f32 => vec![
            Bonus::new(SpellPower::Cold.into_spell_critical_chance(), BonusType::Stacking, 5f32, source!(FrozenWanderer), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::MagicalShelteringCap(), BonusType::Stacking, 10f32, source!(FrozenWanderer), None),
        ]
        4f32 => vec![
            Bonus::new(SpellPower::Cold.into_spell_power(), BonusType::Stacking, 50f32, source!(FrozenWanderer), None),
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
            Bonus::new(SavingThrow::Will.into(), BonusType::Stacking, 2f32, source!(GrandfathersShield), None),
        ]
    )
    TheInevitableGrave "The Inevitable Grave" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::All), BonusType::Stacking, 10f32, source!(TheInevitableGrave), None),
        ]
        3f32 => vec![
            Bonus::new(SavingThrow::Will.into(), BonusType::Stacking, 2f32, source!(TheInevitableGrave), None),
        ]
        4f32 => vec![
            Bonus::new(DifficultyCheck::Spell(SpellSchool::Necromancy.into()).into(), BonusType::Stacking, 2f32, source!(TheInevitableGrave), None)
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
            Bonus::new(DifficultyCheck::Tactics(Tactics::Assassinate).into(), BonusType::Stacking, 2f32, source!(TheLongShadow), None)
        ]
    )
    MelonysMelody "Melody's Melony" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPoints(SpellPoint::Bonus), BonusType::Stacking, 50f32, source!(MelonysMelody), None)
        ]
        3f32 => vec![
            Bonus::new(Ability::Charisma.into(), BonusType::Stacking, 1f32, source!(MelonysMelody), None)
        ]
        4f32 => vec![
            Bonus::new(DifficultyCheck::Spell(SpellSchool::Enchantment.into()).into(), BonusType::Stacking, 2f32, source!(MelonysMelody), None)
        ]
    )
    NystulsMysticalDefense "Nystul's Mystical Defense" => (
        2f32 => vec![
            Bonus::new(Attribute::MagicalSheltering(), BonusType::Stacking, 5f32, source!(NystulsMysticalDefense), None)
        ]
        3f32 => vec![
            Bonus::new(SavingThrow::All.into(), BonusType::Stacking, 1f32, source!(NystulsMysticalDefense), None)
        ]
        4f32 => vec![
            Bonus::new(HitPoint::Bonus.into(), BonusType::Stacking, 100f32, source!(NystulsMysticalDefense), None)
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
            Bonus::new(SpellPoint::Bonus.into(), BonusType::Stacking, 200f32, source!(OttosIrrevocablePower), None)
        ]
        4f32 => vec![
            Bonus::new(DifficultyCheck::Spell(SpellSchool::All.into()).into(), BonusType::Stacking, 2f32, source!(OttosIrrevocablePower), None)
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
            Bonus::new(HealAmp::Positive.into(), BonusType::Stacking, 5f32, source!(Purity), None)
        ]
        3f32 => vec![
            Bonus::immunity(Immunity::MummyRot(), source!(Purity)),
            Bonus::immunity(Immunity::NaturalDisease(), source!(Purity))        ]
        4f32 => vec![
            Bonus::immunity(Immunity::EnergyDrain(), source!(Purity))
        ]
    )
    SnakeBite "Snake Bite" => (
        2f32 => vec![
            Bonus::new(SavingThrow::Poison.into(), BonusType::Stacking, 4f32, source!(SnakeBite), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::ImbueDice(), BonusType::Stacking, 2f32, source!(SnakeBite), None)
        ]
    )
    SpinesOfTheManticore "Spines of the Manticore" => (
        2f32 => vec![
            Bonus::new((WeaponHand::Both, WeaponStat::Damage).into(), BonusType::Stacking, 2f32, source!(SpinesOfTheManticore), None)
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
            Bonus::new(SavingThrow::All.into(), BonusType::Stacking, 1f32, source!(ToHellAndBack), None)
        ]
    )
    TouchOfGrace "Touch of Grace" => (
        2f32 => vec![
            Bonus::new(SpellPoint::Bonus.into(), BonusType::Stacking, 50f32, source!(TouchOfGrace), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Positive), BonusType::Stacking, 20f32, source!(TouchOfGrace), None)
        ]
    )
    TrappersDelight "Trapper's Delight" => (
        2f32 => vec![
            Bonus::new(Skill::OpenLock.into(), BonusType::Stacking, if value >= 4f32 {6f32} else if value == 3f32 {3f32} else {1f32}, source!(TrappersDelight), None),
            Bonus::new(Skill::DisableDevice.into(), BonusType::Stacking, if value >= 4f32 {6f32} else if value == 3f32 {3f32} else {1f32}, source!(TrappersDelight), None),
            Bonus::new(Skill::Search.into(), BonusType::Stacking, if value >= 4f32 {6f32} else if value == 3f32 {3f32} else {1f32}, source!(TrappersDelight), None),
            Bonus::new(Skill::Spot.into(), BonusType::Stacking, if value >= 4f32 {6f32} else if value == 3f32 {3f32} else {1f32}, source!(TrappersDelight), None),
            Bonus::new(SavingThrow::Traps.into(), BonusType::Stacking, if value >= 4f32 {6f32} else if value == 3f32 {3f32} else {1f32}, source!(TrappersDelight), None)
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
            Bonus::new(DifficultyCheck::Tactics(Tactics::Assassinate).into(), BonusType::Stacking, 2f32, source!(Treachery), None)
        ]
    )
    TwilightsCloak "Twilight's Cloak" => (
        2f32 => vec![
            Bonus::new(SavingThrow::Reflex.into(), BonusType::Stacking, 2f32, source!(TwilightsCloak), None)
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
            Bonus::immunity(Immunity::SlipperySurfaces(), source!(Zephyr)),
            Bonus::immunity(Immunity::Knockdown(), source!(Zephyr))
        ]
    )
    BraveryThroughout "Bravery Throughout" => (
        2f32 => vec![
            Bonus::new(SavingThrow::All.into(), BonusType::Stacking, 1f32, source!(BraveryThroughout), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::PhysicalSheltering(), BonusType::Stacking, 5f32, source!(BraveryThroughout), None)
        ]
        4f32 => vec![
            // TODO: 5% Armor Class
        ]
    )
    CoalescedMagic "Coalesced Magic" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Stacking, 10f32, source!(CoalescedMagic), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellCriticalDamage(SpellPower::Universal), BonusType::Stacking, 4f32, source!(CoalescedMagic), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::CasterLevel(SpellType::Arcane.into()), BonusType::Stacking, 1f32, source!(CoalescedMagic), None)
        ]
        5f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Stacking, 30f32, source!(CoalescedMagic), None)
        ]
    )
    CrackshotNegotiator "Crackshot Negotiator" => (
        2f32 => vec![
            Bonus::new(Skill::Diplomacy.into(), BonusType::Stacking, 10f32, source!(CrackshotNegotiator), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::RangedPower(), BonusType::Stacking, 15f32, source!(CrackshotNegotiator), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::Doubleshot(), BonusType::Stacking, 5f32, source!(CrackshotNegotiator), None)
        ]
    )
    DanceOfTheWind "Dance of the Wind" => (
        2f32 => vec![
            Bonus::new(Attribute::MissileDeflection(), BonusType::Stacking, 5f32, source!(DanceOfTheWind), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::MagicalShelteringCap(), BonusType::Stacking, 10f32, source!(DanceOfTheWind), None)
        ]
        4f32 => vec![
            // TODO: Tumble Movement Speed
        ]
    )
    Darkhallow "Darkhallow" => (
        2f32 => vec![
            Bonus::new(HealAmp::Negative.into(), BonusType::Stacking, 20f32, source!(Darkhallow), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Stacking, 20f32, source!(Darkhallow), None)
        ]
        4f32 => vec![
            // TODO: Darkhallow Proc
        ]
    )
    // TODO: Final Burial
    NextFall "Next Fall" => (
        2f32 => vec![
            Bonus::new((WeaponHand::Both, WeaponStat::Attack).into(), BonusType::Stacking, 2f32, source!(NextFall), None),
            Bonus::new((WeaponHand::Both, WeaponStat::Damage).into(), BonusType::Stacking, 2f32, source!(NextFall), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::MeleePower(), BonusType::Stacking, 5f32, source!(NextFall), None),
        ]
        4f32 => vec![
            Bonus::toggle(Toggle::AttackingTrippedTarget(), source!(NextFall)),
            Bonus::new((WeaponHand::Both, WeaponStat::Attack).into(), BonusType::Stacking, 5f32, source!(NextFall), Some(vec![
                Condition::Has(Attribute::Flag(Flag::Toggle(Toggle::AttackingTrippedTarget())))
            ])),
            Bonus::new((WeaponHand::Both, WeaponStat::Damage).into(), BonusType::Stacking, 5f32, source!(NextFall), Some(vec![
                Condition::Has(Attribute::Flag(Flag::Toggle(Toggle::AttackingTrippedTarget())))
            ])),
        ]
    )
    RadiantShield "Radiant Shield" => (
        2f32 => vec![
            Bonus::new(Attribute::ThreatGeneration(ThreatType::Melee), BonusType::Stacking, 50f32, source!(RadiantShield), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::MagicalSheltering(), BonusType::Stacking, 10f32, source!(RadiantShield), None)
        ]
        4f32 => vec![
            Bonus::immunity(Immunity::Quell(), source!(RadiantShield))
        ]
    )
    Reverberation "Reverberation" => (
        2f32 => vec![
            Bonus::new(Attribute::ImbueDice(), BonusType::Stacking, 2f32, source!(Reverberation), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Sonic), BonusType::Stacking, 20f32, source!(Reverberation), None),
        ]
        4f32 => vec![
            Bonus::new(Attribute::SpellCriticalDamage(SpellPower::Sonic), BonusType::Stacking, 3f32, source!(Reverberation), None),
        ]
    )
    SanctifiedFervor "Sanctified Fervor" => (
        2f32 => vec![
            Bonus::new(HealAmp::Positive.into(), BonusType::Stacking, 5f32, source!(SanctifiedFervor), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::MeleePower(), BonusType::Stacking, 5f32, source!(SanctifiedFervor), None),
            Bonus::new(Attribute::RangedPower(), BonusType::Stacking, 5f32, source!(SanctifiedFervor), None),
        ]
        4f32 => vec![
            Bonus::new(Attribute::Doublestrike(), BonusType::Stacking, 2f32, source!(SanctifiedFervor), None),
            Bonus::new(Attribute::Doubleshot(), BonusType::Stacking, 2f32, source!(SanctifiedFervor), None),
        ]
        5f32 => vec![
            // TODO: SmiteEvilBonus
        ]
    )
    ShardsOfMechanus "Shards of Mechanus" => (
        2f32 => vec![
            Bonus::new(HealAmp::Repair.into(), BonusType::Stacking, 20f32, source!(ShardsOfMechanus), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::PhysicalSheltering(), BonusType::Stacking, 10f32, source!(ShardsOfMechanus), None)
        ]
        4f32 => vec![
            // TODO: Repair damage on attack
        ]
    )
    ShatteredDevice "Shattered Device" => (
        2f32 => vec![
            Bonus::new((WeaponHand::Both, WeaponStat::Attack).into(), BonusType::Stacking, 3f32, source!(ShatteredDevice), None),
            Bonus::new((WeaponHand::Both, WeaponStat::Damage).into(), BonusType::Stacking, 3f32, source!(ShatteredDevice), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::Doublestrike(), BonusType::Stacking, 3f32, source!(ShatteredDevice), None),
            Bonus::new(Attribute::Doubleshot(), BonusType::Stacking, 3f32, source!(ShatteredDevice), None),
        ]
        4f32 => vec![
            // TODO: Shattered Device Debuff
        ]
        5f32 => vec![
            // TODO: Shattered Device Debuff
        ]
    )
    Soulweaver "Soulweaver" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Positive), BonusType::Stacking, 10f32, source!(Soulweaver), None),
        ]
        3f32 => vec![
            // TODO: Caster Level with Positive Spells
        ]
        4f32 => vec![
            Bonus::new(SavingThrow::All.into(), BonusType::Stacking, 2f32, source!(Soulweaver), None),
        ]
        5f32 => vec![
            Bonus::new(Attribute::SpellCriticalDamage(SpellPower::Positive), BonusType::Stacking, 5f32, source!(Soulweaver), None),
        ]
    )
    SplendidCacophony "Splendid Cacophony" => (
        2f32 => vec![
            Bonus::new(Attribute::Doublestrike(), BonusType::Stacking, 2f32, source!(SplendidCacophony), None),
            Bonus::new(Attribute::Doubleshot(), BonusType::Stacking, 2f32, source!(SplendidCacophony), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Positive), BonusType::Stacking, 20f32, source!(SplendidCacophony), None)
        ]
        4f32 => vec![
            // TODO: Inspire Courage +1 attack, damage, save vs fear, and +3 universal spell power
        ]
    )
    ThroughTheMists "Through the Mists" => (
        2f32 => vec![
            Bonus::flag(Flag::TrueSeeing(), source!(ThroughTheMists))
        ]
        3f32 => vec![
            Bonus::new(Attribute::Dodge(), BonusType::Stacking, 1f32, source!(ThroughTheMists), None),
            Bonus::new(Attribute::MaxDodge(), BonusType::Stacking, 1f32, source!(ThroughTheMists), None),
        ]
        4f32 => vec![
            Bonus::new(Attribute::RangedPower(), BonusType::Stacking, 10f32, source!(ThroughTheMists), None),
        ]
        5f32 => vec![
            // Uncanny Dodge to 50 ranged power
        ]
    )
    VoltaicExperiment "Voltaic Experiment" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Stacking, 10f32, source!(VoltaicExperiment), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::Doublestrike(), BonusType::Stacking, 2f32, source!(VoltaicExperiment), None),
            Bonus::new(Attribute::Doubleshot(), BonusType::Stacking, 2f32, source!(VoltaicExperiment), None),
        ]
    )
    ZarigansArcaneEnlightenment "Zarigan's Arcane Enlightenment" => (
        2f32 => vec![
            // TODO: Arcane Spell Failure
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Force), BonusType::Stacking, 20f32, source!(ZarigansArcaneEnlightenment), None)
        ]
        4f32 => vec![
            Bonus::new(DifficultyCheck::Spell(SpellSchool::All.into()).into(), BonusType::Stacking, 1f32, source!(ZarigansArcaneEnlightenment), None)
        ]
    )
    TheAbidingPath "The Abiding Path" => (
        2f32 => vec![
            Bonus::new(Attribute::MagicalShelteringCap(), BonusType::Stacking, 10f32, source!(TheAbidingPath), None)
        ]
        3f32 => vec![
            Bonus::new(DifficultyCheck::Tactics(Tactics::Stun).into(), BonusType::Stacking, 2f32, source!(TheAbidingPath), None),
        ]
        4f32 => vec![
            Bonus::new(Attribute::MeleePower(), BonusType::Stacking, 5f32, source!(TheAbidingPath), None),
            Bonus::new(Attribute::RangedPower(), BonusType::Stacking, 5f32, source!(TheAbidingPath), None),
        ]
        5f32 => vec![
            Bonus::new(Attribute::Dodge(), BonusType::Stacking, 1f32, source!(TheAbidingPath), None),
            Bonus::new(Attribute::MaxDodge(), BonusType::Stacking, 1f32, source!(TheAbidingPath), None),
        ]
    )
    AngelicWings "Angelic Wings" => (
        2f32 => vec![
            // TODO: +3 Turn Undead Charges
        ]
        3f32 => vec![
            Bonus::new(Attribute::MagicalSheltering(), BonusType::Stacking, 10f32, source!(AngelicWings), None),
        ]
        4f32 => vec![
            Bonus::new(Attribute::SpellPoints(SpellPoint::Bonus), BonusType::Stacking, 200f32, source!(AngelicWings), None)
        ]
        5f32 => vec![
            Bonus::new(Attribute::SpellCriticalDamage(SpellPower::Light), BonusType::Stacking, 5f32, source!(AngelicWings), None)
        ]
    )
    BendFate "Bend Fate" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Sonic), BonusType::Stacking, 10f32, source!(BendFate), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellPenetration(), BonusType::Stacking, 1f32, source!(BendFate), None)
        ]
        4f32 => vec![
            Bonus::new(DifficultyCheck::Spell(SpellSchool::Enchantment.into()).into(), BonusType::Stacking, 2f32, source!(BendFate), None)
        ]
        5f32 => vec![
            Bonus::new(DifficultyCheck::Spell(SpellSchool::All.into()).into(), BonusType::Stacking, 1f32, source!(BendFate), None)
        ]
    )
    Divinity "Divinity" => (
        2f32 => vec![
            // TODO: Smite Evil Charges
        ]
        3f32 => vec![
            Bonus::new(Attribute::FortificationBypass(), BonusType::Stacking, 10f32, source!(Divinity), None)
        ]
        4f32 => vec![
            Bonus::new(HealAmp::Positive.into(), BonusType::Stacking, 10f32, source!(Divinity), None)
        ]
    )
    Dragonsoul "Dragonsoul" => (
        2f32 => vec![
            // TODO: Spell Cooldowns 5% at 2f32, 10% at 4f32
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellCriticalDamage(SpellPower::All), BonusType::Stacking, if value >= 5f32 {10f32} else {5f32}, source!(Dragonsoul), None)
        ]
    )
    Dreadbringer "Dreadbringer" => (
        2f32 => vec![
            Bonus::new(Attribute::MeleePower(), BonusType::Stacking, if value >= 4f32 {15f32} else {5f32}, source!(Dreadbringer), None),
            Bonus::new(Attribute::RangedPower(), BonusType::Stacking, if value >= 4f32 {15f32} else {5f32}, source!(Dreadbringer), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::FortificationBypass(), BonusType::Stacking, if value >= 5f32 {30f32} else {10f32}, source!(Dreadbringer), None),
        ]
    )
    ElementalAvatar "Elemental Avatar" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::All), BonusType::Stacking, f32::max(5f32, value - 1f32), source!(ElementalAvatar), None)
        ]
    )
    KeeperOfTheCurse "Keeper of the Curse" => (
        2f32 => vec![
            Bonus::new(HealAmp::Positive.into(), BonusType::Stacking, 10f32, source!(KeeperOfTheCurse), None)
        ]
        3f32 => vec![
            Bonus::new(Attribute::MagicalSheltering(), BonusType::Stacking, 10f32, source!(KeeperOfTheCurse), None)
        ]
        4f32 => vec![
            Bonus::new(DifficultyCheck::Spell(SpellSchool::All.into()).into(), BonusType::Stacking, 1f32, source!(KeeperOfTheCurse), None)
        ]
        5f32 => vec![
            // TODO: Pact Dice
        ]
    )
    LunarMagic "Lunar Magic" => (
        2f32 => vec![
            Bonus::new(SpellPoint::Bonus.into(), BonusType::Stacking, 100f32, source!(LunarMagic), None)
        ]
        3f32 => vec![
            Bonus::new(DifficultyCheck::Spell(SpellSchool::All.into()).into(), BonusType::Stacking, if value >= 5f32 {2f32} else {1f32}, source!(LunarMagic), None),
        ]
        4f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Stacking, 20f32, source!(LunarMagic), None),
        ]
    )
    TheSerpent "The Serpent" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Poison), BonusType::Stacking, 20f32, source!(TheSerpent), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellCriticalDamage(SpellPower::Poison), BonusType::Stacking, 5f32, source!(TheSerpent), None),
        ]
        4f32 => vec![
            Bonus::new(DifficultyCheck::Spell(SpellSchool::Transmutation.into()).into(), BonusType::Stacking, 2f32, source!(TheSerpent), None),
            Bonus::new(DifficultyCheck::Spell(SpellSchool::Conjuration.into()).into(), BonusType::Stacking, 2f32, source!(TheSerpent), None),
        ]
        5f32 => vec![
            Bonus::immunity(Immunity::Petrification(), source!(TheSerpent))
        ]
    )
    Shadowstrike "Shadowstrike" => (
        2f32 => vec![
            Bonus::new(DifficultyCheck::Tactics(Tactics::Assassinate).into(), BonusType::Stacking, 2f32, source!(Shadowstrike), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::Dodge(), BonusType::Stacking, 1f32, source!(Shadowstrike), None),
        ]
        4f32 => vec![
            Bonus::new(SavingThrow::Reflex.into(), BonusType::Stacking, 3f32, source!(Shadowstrike), None)
        ]
        5f32 => vec![
            Bonus::new(Attribute::SneakAttackDice(), BonusType::Stacking, 2f32, source!(Shadowstrike), None)
        ]
    )
    Technomage "Technomage" => (
        2f32 => vec![
            Bonus::new(DifficultyCheck::Tactics(Tactics::RuneArm).into(), BonusType::Stacking, 2f32, source!(Technomage), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::Sheltering(), BonusType::Stacking, 10f32, source!(Technomage), None),
        ]
        4f32 => vec![
            // TODO: 5% armor class
        ]
        5f32 => vec![
            // TODO: 20% runearm recharge rate
        ]
    )
    UltimateFury "Ultimate Fury" => (
        2f32 => vec![
            Bonus::new(Ability::Strength.into(), BonusType::Stacking, f32::max(5f32, value - 1f32), source!(UltimateFury), None)
        ]
    )
    Unbreakable "Unbreakable" => (
        2f32 => vec![
            // TODO: +2 Lay on Hands Charges
        ]
        3f32 => vec![
            Bonus::new(Attribute::PhysicalSheltering(), BonusType::Stacking, 10f32, source!(Unbreakable), None),
        ]
        4f32 => vec![
            // TODO: +5 Armor Class
        ]
        5f32 => vec![
            // TODO: +5% Armor Class
        ]
    )
    Wildhunter "Wildhunter" => (
        2f32 => vec![
            Bonus::new(Attribute::RangedPower(), BonusType::Stacking, 5f32, source!(Wildhunter), None)
        ]
        3f32 => vec![
            // TODO: +2 Wild Empathy Cjharges
        ]
        4f32 => vec![
            Bonus::new(Attribute::Doubleshot(), BonusType::Stacking, 5f32, source!(Wildhunter), None),
        ]
        5f32 => vec![
            // TODO: +1 Manyshot Charges
        ]
    )
    SnowpeaksGifts "Snowpeak's Gifts" => (
        2f32 => vec![
            Bonus::new(HealAmp::All.into(), BonusType::Stacking, 10f32, source!(SnowpeaksGifts), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Stacking, 10f32, source!(SnowpeaksGifts), None)
        ]
        4f32 => vec![
            Bonus::immunity(Immunity::MostSlowForms(), source!(SnowpeaksGifts))
        ]
    )
);
