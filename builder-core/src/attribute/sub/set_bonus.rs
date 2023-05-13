use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusType, Condition},
};

use super::{
    Ability, ArmorClass, Defensive, ElementalType, Flag, HealingAmplification, Offensive,
    SavingThrow, Skill, SpellPower, SpellSchool, Threat, Toggle, WeaponHand, WeaponStat, SpellPoint, Health,
};

macro_rules! set_bonuses {
    ($($id: ident $name: expr => ($($count: expr => $bonuses: expr)*))*) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Debug)]
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

        #[cfg(test)]
        mod gen_tests {
            use super::*;

            #[test]
            fn attributes_have_correct_source() {
                $(
                    $(
                        for bonus in $bonuses {
                            assert_eq!(
                                source!($id), bonus.get_source(),
                                "A [{}] bonus has a [{}] source",
                                SetBonus::$id.to_string(),
                                bonus.get_source().to_string()
                            );
                        }
                    )*
                )*
            }
        }
    }
}

macro_rules! source {
    ($set_bonus: ident) => {
        $crate::bonus::BonusSource::Attribute($crate::attribute::Attribute::SetBonus(
            SetBonus::$set_bonus,
        ))
    };
}

set_bonuses!(
    LegendaryMightOfTheAbashai "Legendary Might of the Abashai" => (
        3f32 => vec![
            Bonus::new(Attribute::Defensive(Defensive::Sheltering), BonusType::Profane, 20f32, source!(LegendaryMightOfTheAbashai), None),
            Bonus::new(Attribute::ArmorClass(ArmorClass::Natural), BonusType::Profane, 10f32, source!(LegendaryMightOfTheAbashai), None),
            Bonus::new(Attribute::AbilityScore(Ability::All), BonusType::Profane, 2f32, source!(LegendaryMightOfTheAbashai), None),
            Bonus::new(Attribute::SpellFocus(SpellSchool::All), BonusType::Profane, 2f32, source!(LegendaryMightOfTheAbashai), None),
            Bonus::new(Attribute::HealingAmplification(HealingAmplification::Negative), BonusType::Profane, 10f32, source!(LegendaryMightOfTheAbashai), None),
            Bonus::new(Attribute::HealingAmplification(HealingAmplification::Positive), BonusType::Profane, 10f32, source!(LegendaryMightOfTheAbashai), None),
            Bonus::new(Attribute::HealingAmplification(HealingAmplification::Repair), BonusType::Profane, 10f32, source!(LegendaryMightOfTheAbashai), None),
        ]
        5f32 => vec![
            Bonus::new(Attribute::Defensive(Defensive::Sheltering), BonusType::Profane, 30f32, source!(LegendaryMightOfTheAbashai), None),
            Bonus::new(Attribute::ArmorClass(ArmorClass::Natural), BonusType::Profane, 10f32, source!(LegendaryMightOfTheAbashai), None),
            Bonus::new(Attribute::AbilityScore(Ability::All), BonusType::Profane, 3f32, source!(LegendaryMightOfTheAbashai), None),
            Bonus::new(Attribute::SpellFocus(SpellSchool::All), BonusType::Profane, 3f32, source!(LegendaryMightOfTheAbashai), None),
            Bonus::new(Attribute::HealingAmplification(HealingAmplification::Negative), BonusType::Profane, 30f32, source!(LegendaryMightOfTheAbashai), None),
            Bonus::new(Attribute::HealingAmplification(HealingAmplification::Positive), BonusType::Profane, 30f32, source!(LegendaryMightOfTheAbashai), None),
            Bonus::new(Attribute::HealingAmplification(HealingAmplification::Repair), BonusType::Profane, 30f32, source!(LegendaryMightOfTheAbashai), None),
        ]
    )
    LegendaryDivineBlessing "Legendary Divine Blessing" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Light), BonusType::Artifact, 6f32, source!(LegendaryDivineBlessing), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Light), BonusType::Artifact, 30f32, source!(LegendaryDivineBlessing), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Positive), BonusType::Artifact, 6f32, source!(LegendaryDivineBlessing), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Positive), BonusType::Artifact, 30f32, source!(LegendaryDivineBlessing), None),
        ]
    )
    LegendaryEldersKnowledge "Legendary Elder's Knowledge" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Universal), BonusType::Artifact, 6f32, source!(LegendaryEldersKnowledge), None),
            Bonus::new(Attribute::SpellCriticalDamage(SpellPower::Universal), BonusType::Legendary, 15f32, source!(LegendaryEldersKnowledge), None),
        ]
    )
    LegendaryMarshwalker "Legendary Marshwalker" => (
        2f32 => vec![
            Bonus::new(Attribute::MovementSpeed, BonusType::Enhancement, 30f32, source!(LegendaryMarshwalker), None),
            Bonus::new(Attribute::AbilityScore(Ability::Dexterity), BonusType::Artifact, 3f32, source!(LegendaryMarshwalker), None),
            Bonus::new(Attribute::SavingThrow(SavingThrow::All), BonusType::Artifact, 3f32, source!(LegendaryMarshwalker), None),
            Bonus::new(Attribute::Skill(Skill::Jump), BonusType::Artifact, 7f32, source!(LegendaryMarshwalker), None),
            Bonus::new(Attribute::Skill(Skill::Tumble), BonusType::Artifact, 7f32, source!(LegendaryMarshwalker), None),
        ]
    )
    LegendaryRavensEye "Legendary Raven's Eye" => (
        2f32 => vec![
            Bonus::new(Attribute::WeaponStat(WeaponHand::Both, WeaponStat::Attack), BonusType::Artifact, 3f32, source!(LegendaryRavensEye), None),
            Bonus::new(Attribute::WeaponStat(WeaponHand::Both, WeaponStat::Damage), BonusType::Artifact, 3f32, source!(LegendaryRavensEye), None),
            Bonus::new(Attribute::WeaponStat(WeaponHand::Both, WeaponStat::CriticalAttack), BonusType::Artifact, 3f32, source!(LegendaryRavensEye), None),
            Bonus::new(Attribute::WeaponStat(WeaponHand::Both, WeaponStat::CriticalDamage), BonusType::Artifact, 3f32, source!(LegendaryRavensEye), None),
            Bonus::new(Attribute::Offensive(Offensive::SneakAttackAttack), BonusType::Artifact, 3f32, source!(LegendaryRavensEye), None),
            Bonus::new(Attribute::Offensive(Offensive::SneakAttackDamage), BonusType::Artifact, 3f32, source!(LegendaryRavensEye), None),
            Bonus::new(Attribute::Skill(Skill::Search), BonusType::Artifact, 7f32, source!(LegendaryRavensEye), None),
            Bonus::new(Attribute::Skill(Skill::Spot), BonusType::Artifact, 7f32, source!(LegendaryRavensEye), None),
        ]
    )
    LegendaryShamansFury "Legendary Shaman's Fury" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Fire), BonusType::Artifact, 6f32, source!(LegendaryShamansFury), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Fire), BonusType::Artifact, 30f32, source!(LegendaryShamansFury), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Cold), BonusType::Artifact, 6f32, source!(LegendaryShamansFury), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Cold), BonusType::Artifact, 30f32, source!(LegendaryShamansFury), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Electric), BonusType::Artifact, 6f32, source!(LegendaryShamansFury), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Electric), BonusType::Artifact, 30f32, source!(LegendaryShamansFury), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Acid), BonusType::Artifact, 6f32, source!(LegendaryShamansFury), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Acid), BonusType::Artifact, 30f32, source!(LegendaryShamansFury), None),
        ]
    )
    LegendarySirensWard "Legendary Siren's Ward" => (
        2f32 => vec![
            Bonus::new(Attribute::ArmorClass(ArmorClass::Scalar), BonusType::Artifact, 0.15f32, source!(LegendarySirensWard), None),
            Bonus::new(Attribute::SavingThrow(SavingThrow::All), BonusType::Artifact, 3f32, source!(LegendarySirensWard), None),
            Bonus::new(Attribute::ArmorClass(ArmorClass::Shield), BonusType::Artifact, 3f32, source!(LegendarySirensWard), None),
        ]
    )
    LegendaryVulkoorsCunning "Legendary Vulkoor's Cunning" => (
        2f32 => vec![
            Bonus::new(Attribute::ThreatMultipler(Threat::Melee), BonusType::Artifact, -0.2f32, source!(LegendaryVulkoorsCunning), None),
            Bonus::new(Attribute::Offensive(Offensive::MeleePower), BonusType::Artifact, 15f32, source!(LegendaryVulkoorsCunning), None),
            Bonus::new(Attribute::Offensive(Offensive::RangedPower), BonusType::Artifact, 15f32, source!(LegendaryVulkoorsCunning), None),
            Bonus::new(Attribute::Flag(Flag::Simple(crate::attribute::SimpleFlag::VulkoorCunningProc)), BonusType::Stacking, 1f32, source!(LegendaryVulkoorsCunning), None),
        ]
    )
    LegendaryVulkoorsMight "Legendary Vulkoor's  Might" => (
        2f32 => vec![
            Bonus::new(Attribute::ThreatMultipler(Threat::All), BonusType::Artifact, 0.2f32, source!(LegendaryVulkoorsMight), None),
            Bonus::new(Attribute::WeaponStat(WeaponHand::Both, WeaponStat::Attack), BonusType::Artifact, 3f32, source!(LegendaryVulkoorsMight), None),
            Bonus::new(Attribute::WeaponStat(WeaponHand::Both, WeaponStat::Damage), BonusType::Artifact, 3f32, source!(LegendaryVulkoorsMight), None),
            Bonus::new(Attribute::WeaponStat(WeaponHand::Both, WeaponStat::CriticalAttack), BonusType::Artifact, 3f32, source!(LegendaryVulkoorsMight), None),
            Bonus::new(Attribute::WeaponStat(WeaponHand::Both, WeaponStat::CriticalDamage), BonusType::Artifact, 3f32, source!(LegendaryVulkoorsMight), None),
            Bonus::new(Attribute::Offensive(Offensive::SneakAttackAttack), BonusType::Artifact, 3f32, source!(LegendaryVulkoorsMight), None),
            Bonus::new(Attribute::Offensive(Offensive::SneakAttackDamage), BonusType::Artifact, 3f32, source!(LegendaryVulkoorsMight), None),
        ]
    )
    LegendaryWrathOfSoraKell "Legendary Wrath of Sora Kell" => (
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Artifact, 25f32, source!(LegendaryWrathOfSoraKell), None),
            Bonus::new(Attribute::Defensive(Defensive::PhysicalSheltering), BonusType::Artifact, 30f32, source!(LegendaryWrathOfSoraKell), None),
            Bonus::new(Attribute::Offensive(Offensive::ImbueDice), BonusType::Artifact, 3f32, source!(LegendaryWrathOfSoraKell), None),
        ]
    )
    LegendaryPerfectedWrath "Legendary Perfected Wrath" => (
        3f32 => vec![
            Bonus::new(Attribute::AbilityScore(Ability::All), BonusType::Artifact, 3f32, source!(LegendaryPerfectedWrath), None),
            Bonus::new(Attribute::Offensive(Offensive::HelplessDamage), BonusType::Artifact, 15f32, source!(LegendaryPerfectedWrath), None),
        ]
    )
    LegendaryBeaconOfMagic "Legendary Beacon of Magic" => (
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Artifact, 25f32, source!(LegendaryBeaconOfMagic), None),
            Bonus::new(Attribute::SpellFocus(SpellSchool::All), BonusType::Artifact, 3f32, source!(LegendaryBeaconOfMagic), None),
            Bonus::new(Attribute::Defensive(Defensive::MagicalSheltering), BonusType::Artifact, 30f32, source!(LegendaryBeaconOfMagic), None),
            Bonus::new(Attribute::Defensive(Defensive::MagicalShelteringCap), BonusType::Artifact, 30f32, source!(LegendaryBeaconOfMagic), None),
            Bonus::new(Attribute::Defensive(Defensive::MissileDeflection), BonusType::Artifact, 5f32, source!(LegendaryBeaconOfMagic), None),
        ]
    )
    LegendaryKnightOfTheShadows "Legendary Knight of the Shadows" => (
        3f32 => vec![
            Bonus::new(Attribute::Defensive(Defensive::Sheltering), BonusType::Artifact, 30f32, source!(LegendaryKnightOfTheShadows), None),
            Bonus::new(Attribute::ThreatMultipler(Threat::Melee), BonusType::Artifact, 0.5f32, source!(LegendaryKnightOfTheShadows), None),
            Bonus::new(Attribute::ThreatMultipler(Threat::Ranged), BonusType::Artifact, 0.5f32, source!(LegendaryKnightOfTheShadows), None),
            Bonus::new(Attribute::ArmorClass(ArmorClass::Scalar), BonusType::Artifact, 0.15f32, source!(LegendaryKnightOfTheShadows), None),
        ]
    )
    LegendaryCrpytRaider "Legendary Crypt Raider" => (
        3f32 => vec![
            Bonus::new(Attribute::Toggle(super::Toggle::AttackingEvilCreatures), BonusType::Stacking, 1f32, source!(LegendaryCrpytRaider), None),
            Bonus::new(Attribute::WeaponStat(WeaponHand::Both, WeaponStat::Attack), BonusType::Stacking, 5f32, source!(LegendaryCrpytRaider), Some(vec![
                Condition::Has(Attribute::Flag(Flag::Toggle(super::Toggle::AttackingEvilCreatures)))
            ])),
            Bonus::new(Attribute::WeaponStat(WeaponHand::Both, WeaponStat::Damage), BonusType::Stacking, 5f32, source!(LegendaryCrpytRaider), Some(vec![
                Condition::Has(Attribute::Flag(Flag::Toggle(super::Toggle::AttackingEvilCreatures)))
            ])),
            Bonus::new(Attribute::SavingThrow(SavingThrow::All), BonusType::Stacking, 3f32, source!(LegendaryCrpytRaider), Some(vec![
                Condition::Has(Attribute::Flag(Flag::Toggle(super::Toggle::AttackingEvilCreatures)))
            ])),
            Bonus::new(Attribute::Offensive(Offensive::MeleePower), BonusType::Artifact, 15f32, source!(LegendaryCrpytRaider), None),
            Bonus::new(Attribute::Offensive(Offensive::RangedPower), BonusType::Artifact, 15f32, source!(LegendaryCrpytRaider), None),
            Bonus::new(Attribute::ThreatMultipler(Threat::Ranged), BonusType::Artifact, -0.2f32, source!(LegendaryCrpytRaider), None),
            Bonus::new(Attribute::ThreatMultipler(Threat::Melee), BonusType::Artifact, -0.2f32, source!(LegendaryCrpytRaider), None),
            Bonus::new(Attribute::Offensive(Offensive::ImbueDice), BonusType::Artifact, 3f32, source!(LegendaryCrpytRaider), None),
        ]
    )
    LegendarySilentAvenger "Legendary Silent Avenger" => (
        3f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::Doublestrike), BonusType::Artifact, 15f32, source!(LegendarySilentAvenger), None),
            Bonus::new(Attribute::Offensive(Offensive::Doubleshot), BonusType::Artifact, 15f32, source!(LegendarySilentAvenger), None),
            Bonus::new(Attribute::Offensive(Offensive::SneakAttackDice), BonusType::Artifact, 3f32, source!(LegendarySilentAvenger), None),
            Bonus::new(Attribute::Offensive(Offensive::FortificationBypass), BonusType::Artifact, 30f32, source!(LegendarySilentAvenger), None),
            Bonus::new(Attribute::Offensive(Offensive::HelplessDamage), BonusType::Artifact, 15f32, source!(LegendarySilentAvenger), None),
        ]
    )
    LegendaryAdherentOfTheMists "Legendary Adherent of the Mists" => (
        5f32 => vec![
            Bonus::new(Attribute::Defensive(Defensive::PhysicalSheltering), BonusType::Profane, 30f32, source!(LegendaryAdherentOfTheMists), None),
            Bonus::new(Attribute::HealingAmplification(HealingAmplification::Negative), BonusType::Profane, 30f32, source!(LegendaryAdherentOfTheMists), None),
            Bonus::new(Attribute::HealingAmplification(HealingAmplification::Positive), BonusType::Profane, 30f32, source!(LegendaryAdherentOfTheMists), None),
            Bonus::new(Attribute::Offensive(Offensive::MeleePower), BonusType::Profane, 15f32, source!(LegendaryAdherentOfTheMists), None),
            Bonus::new(Attribute::Offensive(Offensive::RangedPower), BonusType::Profane, 15f32, source!(LegendaryAdherentOfTheMists), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Profane, 15f32, source!(LegendaryAdherentOfTheMists), None),
        ]
    )
    LegendaryWaywardWarrior "Legendary Wayward Warrior" => (
        2f32 => vec![
            Bonus::new(Attribute::ArmorClass(ArmorClass::Natural), BonusType::Artifact, 5f32, source!(LegendaryWaywardWarrior), None),
            Bonus::new(Attribute::AbilityScore(Ability::Constitution), BonusType::Artifact, 3f32, source!(LegendaryWaywardWarrior), None),
        ]
    )
    LegendarySeasonsOfChange "Legendary Seasons of Change" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Fire), BonusType::Artifact, 30f32, source!(LegendarySeasonsOfChange), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Cold), BonusType::Artifact, 30f32, source!(LegendarySeasonsOfChange), None),
            Bonus::new(Attribute::SpellFocus(SpellSchool::Evocation), BonusType::Artifact, 3f32, source!(LegendarySeasonsOfChange), None),
        ]
    )
    LegendaryRenegadeChampion "Legendary Renegade Champion" => (
        3f32 => vec![
            Bonus::new(Attribute::WeaponStat(WeaponHand::Both, WeaponStat::CriticalMultiplier1920), BonusType::Artifact, 1f32, source!(LegendaryRenegadeChampion), None),
            // TODO: Rune Arm DCs
            Bonus::new(Attribute::HealingAmplification(HealingAmplification::Repair), BonusType::Artifact, 30f32, source!(LegendaryRenegadeChampion), None),
        ]
    )
    LegendaryHeavyWarfare "Legendary Heavy Warfare" => (
        2f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::MeleePower), BonusType::Artifact, 15f32, source!(LegendaryHeavyWarfare), None),
            Bonus::new(Attribute::Offensive(Offensive::Doublestrike), BonusType::Artifact, 15f32, source!(LegendaryHeavyWarfare), None),
        ]
    )
    LegendaryCurseNecromancer "Legendary Cursed Necromancer" => (
        2f32 => vec![
            Bonus::new(Attribute::HealingAmplification(HealingAmplification::Negative), BonusType::Artifact, 30f32, source!(LegendaryCurseNecromancer), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Negative), BonusType::Artifact, 6f32, source!(LegendaryCurseNecromancer), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Negative), BonusType::Artifact, 30f32, source!(LegendaryCurseNecromancer), None),
            // TODO: Legendary Dreadful Curse
        ]
    )
    BrilliantCrescents "Brilliant Crescents" => (
        2f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::OffHandAttackChance), BonusType::Stacking, 20f32, source!(BrilliantCrescents), None)
        ]
    )
    MountainskinSet "Mountainskin Set" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Acid), BonusType::Artifact, 30f32, source!(MountainskinSet), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Acid), BonusType::Artifact, 6f32, source!(MountainskinSet), None),
            // TODO: Defensive Stoneskin Chance
        ]
    )
    LegendaryArcsteelBattlemage "Legendary Arcsteel Battlemage" => (
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Electric), BonusType::Artifact, 30f32, source!(LegendaryArcsteelBattlemage), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Fire), BonusType::Artifact, 30f32, source!(LegendaryArcsteelBattlemage), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Force), BonusType::Artifact, 30f32, source!(LegendaryArcsteelBattlemage), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Repair), BonusType::Artifact, 30f32, source!(LegendaryArcsteelBattlemage), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Electric), BonusType::Artifact, 6f32, source!(LegendaryArcsteelBattlemage), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Fire), BonusType::Artifact, 6f32, source!(LegendaryArcsteelBattlemage), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Force), BonusType::Artifact, 6f32, source!(LegendaryArcsteelBattlemage), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Repair), BonusType::Artifact, 6f32, source!(LegendaryArcsteelBattlemage), None),
            Bonus::new(Attribute::AbilityScore(Ability::Intelligence), BonusType::Artifact, 3f32, source!(LegendaryArcsteelBattlemage), None),
            Bonus::new(Attribute::SpellFocus(SpellSchool::All), BonusType::Artifact, 3f32, source!(LegendaryArcsteelBattlemage), None),
        ]
    )
    LegendaryEsotericInitiate "Legendary Esoteric Initiate" => (
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Artifact, 25f32, source!(LegendaryEsotericInitiate), None),
            Bonus::new(Attribute::AbilityScore(Ability::Intelligence), BonusType::Artifact, 3f32, source!(LegendaryEsotericInitiate), None),
            Bonus::new(Attribute::AbilityScore(Ability::Charisma), BonusType::Artifact, 3f32, source!(LegendaryEsotericInitiate), None),
            Bonus::new(Attribute::AbilityScore(Ability::Wisdom), BonusType::Artifact, 3f32, source!(LegendaryEsotericInitiate), None),
            Bonus::new(Attribute::SpellFocus(SpellSchool::All), BonusType::Artifact, 3f32, source!(LegendaryEsotericInitiate), None),
            Bonus::new(Attribute::Defensive(Defensive::MagicalShelteringCap), BonusType::Artifact, 30f32, source!(LegendaryEsotericInitiate), None),
        ]
    )
    LegendaryFlameclensedFury "Legendary Flameclensed Fury" => (
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Fire), BonusType::Artifact, 30f32, source!(LegendaryFlameclensedFury), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Force), BonusType::Artifact, 30f32, source!(LegendaryFlameclensedFury), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Light), BonusType::Artifact, 30f32, source!(LegendaryFlameclensedFury), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Positive), BonusType::Artifact, 30f32, source!(LegendaryFlameclensedFury), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Fire), BonusType::Artifact, 6f32, source!(LegendaryFlameclensedFury), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Force), BonusType::Artifact, 6f32, source!(LegendaryFlameclensedFury), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Light), BonusType::Artifact, 6f32, source!(LegendaryFlameclensedFury), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Positive), BonusType::Artifact, 6f32, source!(LegendaryFlameclensedFury), None),
            Bonus::new(Attribute::AbilityScore(Ability::Wisdom), BonusType::Artifact, 3f32, source!(LegendaryFlameclensedFury), None),
            Bonus::new(Attribute::AbilityScore(Ability::Charisma), BonusType::Artifact, 3f32, source!(LegendaryFlameclensedFury), None),
        ]
    )
    LegendaryGuardianOfTheGates "Legendary Guarrdian of the Gates" => (
        3f32 => vec![
            Bonus::new(Attribute::ArmorClass(ArmorClass::Scalar), BonusType::Artifact, 0.15f32, source!(LegendaryGuardianOfTheGates), None),
            Bonus::new(Attribute::Defensive(Defensive::Sheltering), BonusType::Artifact, 30f32, source!(LegendaryGuardianOfTheGates), None),
            Bonus::new(Attribute::ThreatMultipler(Threat::Melee), BonusType::Artifact, 75f32, source!(LegendaryGuardianOfTheGates), None),
            Bonus::new(Attribute::ElementalAbsortion(ElementalType::Fire), BonusType::Artifact, 10f32, source!(LegendaryGuardianOfTheGates), None),
        ]
    )
    LegendaryHruitsInfluence "Legendary Hruit's Influence" => (
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Fire), BonusType::Artifact, 30f32, source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Cold), BonusType::Artifact, 30f32, source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Electric), BonusType::Artifact, 30f32, source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Positive), BonusType::Artifact, 30f32, source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Fire), BonusType::Artifact, 6f32, source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Cold), BonusType::Artifact, 6f32, source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Electric), BonusType::Artifact, 6f32, source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Positive), BonusType::Artifact, 6f32, source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::AbilityScore(Ability::Wisdom), BonusType::Artifact, 3f32, source!(LegendaryHruitsInfluence), None),
            Bonus::new(Attribute::SpellFocus(SpellSchool::All), BonusType::Artifact, 3f32, source!(LegendaryHruitsInfluence), None),
        ]
    )
    LegendaryPartOfTheFamily "Legendary Part of the Family" => (
        3f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::Doublestrike), BonusType::Artifact, 15f32, source!(LegendaryPartOfTheFamily), None),
            Bonus::new(Attribute::Offensive(Offensive::MeleePower), BonusType::Artifact, 15f32, source!(LegendaryPartOfTheFamily), None),
            Bonus::new(Attribute::Offensive(Offensive::HelplessDamage), BonusType::Artifact, 15f32, source!(LegendaryPartOfTheFamily), None),
            Bonus::new(Attribute::Offensive(Offensive::FortificationBypass), BonusType::Artifact, 30f32, source!(LegendaryPartOfTheFamily), None),
        ]
    )
    LegendaryWallwatch "Legendary Wallwatch" => (
        3f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::SneakAttackDice), BonusType::Artifact, 3f32, source!(LegendaryWallwatch), None),
            Bonus::new(Attribute::Offensive(Offensive::FortificationBypass), BonusType::Artifact, 30f32, source!(LegendaryWallwatch), None),
            Bonus::new(Attribute::Offensive(Offensive::Doubleshot), BonusType::Artifact, 15f32, source!(LegendaryWallwatch), None),
            Bonus::new(Attribute::Offensive(Offensive::RangedPower), BonusType::Artifact, 15f32, source!(LegendaryWallwatch), None),
        ]
    )
    LegendaryDreadkeeper "Legendary Dreadkeeper" => (
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Negative), BonusType::Artifact, 30f32, source!(LegendaryDreadkeeper), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Poison), BonusType::Artifact, 30f32, source!(LegendaryDreadkeeper), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Force), BonusType::Artifact, 30f32, source!(LegendaryDreadkeeper), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Negative), BonusType::Artifact, 6f32, source!(LegendaryDreadkeeper), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Poison), BonusType::Artifact, 6f32, source!(LegendaryDreadkeeper), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Force), BonusType::Artifact, 6f32, source!(LegendaryDreadkeeper), None),
            Bonus::new(Attribute::AbilityScore(Ability::Intelligence), BonusType::Artifact, 3f32, source!(LegendaryDreadkeeper), None),
            Bonus::new(Attribute::Defensive(Defensive::PhysicalSheltering), BonusType::Artifact, 20f32, source!(LegendaryDreadkeeper), None),
            Bonus::new(Attribute::SpellFocus(SpellSchool::All), BonusType::Artifact, 3f32, source!(LegendaryDreadkeeper), None),
        ]
    )
    LegendaryFeywildDreramer "Legendary Feywild Dreamer" => (
        3f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Sonic), BonusType::Artifact, 30f32, source!(LegendaryFeywildDreramer), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Force), BonusType::Artifact, 30f32, source!(LegendaryFeywildDreramer), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Light), BonusType::Artifact, 30f32, source!(LegendaryFeywildDreramer), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Acid), BonusType::Artifact, 30f32, source!(LegendaryFeywildDreramer), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Sonic), BonusType::Artifact, 6f32, source!(LegendaryFeywildDreramer), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Force), BonusType::Artifact, 6f32, source!(LegendaryFeywildDreramer), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Light), BonusType::Artifact, 6f32, source!(LegendaryFeywildDreramer), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Acid), BonusType::Artifact, 6f32, source!(LegendaryFeywildDreramer), None),
            Bonus::new(Attribute::AbilityScore(Ability::Charisma), BonusType::Artifact, 3f32, source!(LegendaryFeywildDreramer), None),
            Bonus::new(Attribute::SpellFocus(SpellSchool::All), BonusType::Artifact, 3f32, source!(LegendaryFeywildDreramer), None),
        ]
    )
    LegendaryProfaneExperiment "Legendary Profane Experiment" => (
        3f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::Doublestrike), BonusType::Artifact, 15f32, source!(LegendaryProfaneExperiment), None),
            Bonus::new(Attribute::Offensive(Offensive::Doubleshot), BonusType::Artifact, 15f32, source!(LegendaryProfaneExperiment), None),
            Bonus::new(Attribute::AbilityScore(Ability::Constitution), BonusType::Artifact, 3f32, source!(LegendaryProfaneExperiment), None),
            Bonus::new(Attribute::AbilityScore(Ability::Intelligence), BonusType::Artifact, 3f32, source!(LegendaryProfaneExperiment), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Artifact, 25f32, source!(LegendaryProfaneExperiment), None),
            Bonus::new(Attribute::Offensive(Offensive::ImbueDice), BonusType::Artifact, 3f32, source!(LegendaryProfaneExperiment), None),
        ]
    )
    LegacyOfLorikk "Legacy of Lorikk" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Positive), BonusType::Artifact, 30f32, source!(LegacyOfLorikk), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Light), BonusType::Artifact, 30f32, source!(LegacyOfLorikk), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Positive), BonusType::Artifact, 6f32, source!(LegacyOfLorikk), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Light), BonusType::Artifact, 6f32, source!(LegacyOfLorikk), None),
        ]
    )
    LegacyOfLevikk "Legacy of Levikk" => (
        2f32 => vec![
            Bonus::new(Attribute::ArmorClass(ArmorClass::Scalar), BonusType::Artifact, 0.15f32, source!(LegacyOfLevikk), None),
            Bonus::new(Attribute::Defensive(Defensive::PhysicalSheltering), BonusType::Artifact, 30f32, source!(LegacyOfLevikk), None),
            Bonus::new(Attribute::ThreatMultipler(Threat::Melee), BonusType::Artifact, 75f32, source!(LegacyOfLevikk), None),
        ]
    )
    MindAndMatter "Mind and Matter" => (
        2f32 => vec![
            Bonus::new(Attribute::Defensive(Defensive::Sheltering), BonusType::Artifact, 30f32, source!(MindAndMatter), None),
            Bonus::new(Attribute::Defensive(Defensive::MagicalShelteringCap), BonusType::Artifact, 30f32, source!(MindAndMatter), None),
        ]
    )
    LegacyOfTharne "Legacy of Tharne" => (
        2f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::Doublestrike), BonusType::Artifact, 15f32, source!(LegacyOfTharne), None),
            Bonus::new(Attribute::Offensive(Offensive::Doubleshot), BonusType::Artifact, 15f32, source!(LegacyOfTharne), None),
            Bonus::new(Attribute::Offensive(Offensive::SneakAttackDice), BonusType::Artifact, 3f32, source!(LegacyOfTharne), None),
            Bonus::new(Attribute::Skill(Skill::Spot), BonusType::Artifact, 5f32, source!(LegacyOfTharne), None),
            Bonus::new(Attribute::Skill(Skill::Search), BonusType::Artifact, 5f32, source!(LegacyOfTharne), None),
        ]
    )
    AngerOfTheAvalanche "Anger of the Avalanche" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Cold), BonusType::Artifact, 30f32, source!(AngerOfTheAvalanche), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Sonic), BonusType::Artifact, 30f32, source!(AngerOfTheAvalanche), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Electric), BonusType::Artifact, 30f32, source!(AngerOfTheAvalanche), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Cold), BonusType::Artifact, 6f32, source!(AngerOfTheAvalanche), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Sonic), BonusType::Artifact, 6f32, source!(AngerOfTheAvalanche), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Electric), BonusType::Artifact, 6f32, source!(AngerOfTheAvalanche), None),
        ]
    )
    MantleOfSuulomades "Mantle of Suulomades" => (
        2f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::Doublestrike), BonusType::Artifact, 15f32, source!(MantleOfSuulomades), None),
            Bonus::new(Attribute::Offensive(Offensive::Doubleshot), BonusType::Artifact, 15f32, source!(MantleOfSuulomades), None),
            Bonus::new(Attribute::Defensive(Defensive::Sheltering), BonusType::Artifact, 30f32, source!(MantleOfSuulomades), None),
        ]
    )
    ChainedElementals "Chained Elementals" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Fire), BonusType::Artifact, 30f32, source!(ChainedElementals), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Cold), BonusType::Artifact, 30f32, source!(ChainedElementals), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Acid), BonusType::Artifact, 30f32, source!(ChainedElementals), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Electric), BonusType::Artifact, 30f32, source!(ChainedElementals), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Fire), BonusType::Artifact, 6f32, source!(ChainedElementals), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Cold), BonusType::Artifact, 6f32, source!(ChainedElementals), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Acid), BonusType::Artifact, 6f32, source!(ChainedElementals), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Electric), BonusType::Artifact, 6f32, source!(ChainedElementals), None),
        ]
    )
    TyrannicalTinkerer "Tyrannical Tinkerer" => (
        2f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::MeleePower), BonusType::Artifact, 15f32, source!(TyrannicalTinkerer), None),
            Bonus::new(Attribute::Offensive(Offensive::RangedPower), BonusType::Artifact, 15f32, source!(TyrannicalTinkerer), None),
            Bonus::new(Attribute::Offensive(Offensive::FortificationBypass), BonusType::Artifact, 30f32, source!(TyrannicalTinkerer), None),
            Bonus::new(Attribute::Skill(Skill::OpenLock), BonusType::Artifact, 5f32, source!(TyrannicalTinkerer), None),
            Bonus::new(Attribute::Skill(Skill::DisableDevice), BonusType::Artifact, 5f32, source!(TyrannicalTinkerer), None),
        ]
    )
    MasterfulMagewright "Masterful Magewright" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellFocus(SpellSchool::All), BonusType::Artifact, 3f32, source!(MasterfulMagewright), None),
            Bonus::new(Attribute::AbilityScore(Ability::Intelligence), BonusType::Artifact, 3f32, source!(MasterfulMagewright), None),
            Bonus::new(Attribute::AbilityScore(Ability::Wisdom), BonusType::Artifact, 3f32, source!(MasterfulMagewright), None),
            Bonus::new(Attribute::AbilityScore(Ability::Charisma), BonusType::Artifact, 3f32, source!(MasterfulMagewright), None),
            Bonus::new(Attribute::Skill(Skill::Perform), BonusType::Artifact, 5f32, source!(MasterfulMagewright), None),
            Bonus::new(Attribute::Skill(Skill::Concentration), BonusType::Artifact, 5f32, source!(MasterfulMagewright), None),
        ]
    )
    FastidiousFabricator "fastiditous Fabricator" => (
        2f32 => vec![
            Bonus::new(Attribute::Defensive(Defensive::MagicalSheltering), BonusType::Artifact, 30f32, source!(FastidiousFabricator), None),
            Bonus::new(Attribute::Defensive(Defensive::MagicalShelteringCap), BonusType::Artifact, 30f32, source!(FastidiousFabricator), None),
            Bonus::new(Attribute::Skill(Skill::Balance), BonusType::Artifact, 5f32, source!(FastidiousFabricator), None),
            Bonus::new(Attribute::Skill(Skill::Repair), BonusType::Artifact, 5f32, source!(FastidiousFabricator), None),
        ]
    )
    AstuteAlchemist "Astute Alchemist" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Fire), BonusType::Artifact, 30f32, source!(AstuteAlchemist), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Poison), BonusType::Artifact, 30f32, source!(AstuteAlchemist), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Acid), BonusType::Artifact, 30f32, source!(AstuteAlchemist), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Fire), BonusType::Artifact, 6f32, source!(AstuteAlchemist), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Poison), BonusType::Artifact, 6f32, source!(AstuteAlchemist), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Acid), BonusType::Artifact, 6f32, source!(AstuteAlchemist), None),
        ]
    )
    ConduitOfTheTitans "Conduit of the Titans" => (
        2f32 => vec![
            // TODO: Rune arm DCs
            Bonus::new(Attribute::SpellPower(SpellPower::Electric), BonusType::Artifact, 30f32, source!(ConduitOfTheTitans), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Repair), BonusType::Artifact, 30f32, source!(ConduitOfTheTitans), None),
            Bonus::new(Attribute::SpellPower(SpellPower::Force), BonusType::Artifact, 30f32, source!(ConduitOfTheTitans), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Electric), BonusType::Artifact, 6f32, source!(ConduitOfTheTitans), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Repair), BonusType::Artifact, 6f32, source!(ConduitOfTheTitans), None),
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Force), BonusType::Artifact, 6f32, source!(ConduitOfTheTitans), None),
        ]
    )
    EminenceOfAutumn "Eminence of Autumn" => (
        2f32 => vec![
            Bonus::new(Attribute::SpellPower(SpellPower::Universal), BonusType::Artifact, 25f32, source!(EminenceOfAutumn), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::SpellCriticalChance(SpellPower::Universal), BonusType::Artifact, 6f32, source!(EminenceOfAutumn), None),
        ]
        4f32 => vec![
            Bonus::new(Attribute::SpellPoints(SpellPoint::Scalar), BonusType::Legendary, 0.1f32, source!(EminenceOfAutumn), None)
        ]
        5f32 => vec![
            Bonus::new(Attribute::AbilityScore(Ability::Intelligence), BonusType::Artifact, 3f32, source!(EminenceOfAutumn), None),
            Bonus::new(Attribute::AbilityScore(Ability::Wisdom), BonusType::Artifact, 3f32, source!(EminenceOfAutumn), None),
            Bonus::new(Attribute::AbilityScore(Ability::Charisma), BonusType::Artifact, 3f32, source!(EminenceOfAutumn), None),
        ]
        6f32 => vec![
            Bonus::new(Attribute::SpellFocus(SpellSchool::All), BonusType::Artifact, 3f32, source!(EminenceOfAutumn), None),
        ]
        7f32 => vec![
            Bonus::new(Attribute::SpellCriticalDamage(SpellPower::Universal), BonusType::Legendary, 15f32, source!(EminenceOfAutumn), None)
        ]
    )
    EminenceOfSpring "Eminence of Spring" => (
        2f32 => vec![
            Bonus::new(Attribute::Defensive(Defensive::MissileDeflection), BonusType::Artifact, 10f32, source!(EminenceOfSpring), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::HelplessDamage), BonusType::Artifact, 15f32, source!(EminenceOfSpring), None),
        ]
        4f32 => Vec::<Bonus>::new() // TODO: Dodge Cap
        5f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::SneakAttackDice), BonusType::Artifact, 3f32, source!(EminenceOfSpring), None),
        ]
        6f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::Doubleshot), BonusType::Artifact, 15f32, source!(EminenceOfSpring), None),
            Bonus::new(Attribute::Offensive(Offensive::Doublestrike), BonusType::Artifact, 15f32, source!(EminenceOfSpring), None),
        ]
        7f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::MeleePower), BonusType::Artifact, 15f32, source!(EminenceOfSpring), None),
            Bonus::new(Attribute::Offensive(Offensive::RangedPower), BonusType::Artifact, 15f32, source!(EminenceOfSpring), None),
        ]
    )
    EminenceOfSummer "Eminence of Summer" => (
        2f32 => vec![
            Bonus::new(Attribute::Defensive(Defensive::MagicalSheltering), BonusType::Artifact, 30f32, source!(EminenceOfSummer), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::FortificationBypass), BonusType::Artifact, 30f32, source!(EminenceOfSummer), None),
        ]
        // TODO: Tactical Abilities
        5f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::HelplessDamage), BonusType::Artifact, 15f32, source!(EminenceOfSummer), None)
        ]
        6f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::Doublestrike), BonusType::Artifact, 15f32, source!(EminenceOfSummer), None),
            Bonus::new(Attribute::Offensive(Offensive::Doubleshot), BonusType::Artifact, 15f32, source!(EminenceOfSummer), None),
        ]
        7f32 => vec![
            Bonus::new(Attribute::Offensive(Offensive::MeleePower), BonusType::Artifact, 15f32, source!(EminenceOfSummer), None),
            Bonus::new(Attribute::Offensive(Offensive::RangedPower), BonusType::Artifact, 15f32, source!(EminenceOfSummer), None),
        ]
    )
    EminenceOfWinter "Eminence of Winter" => (
        2f32 => vec![
            Bonus::new(Attribute::Defensive(Defensive::PhysicalSheltering), BonusType::Artifact, 30f32, source!(EminenceOfWinter), None),
        ]
        3f32 => vec![
            Bonus::new(Attribute::HealingAmplification(HealingAmplification::Positive), BonusType::Artifact, 30f32, source!(EminenceOfWinter), None),
            Bonus::new(Attribute::HealingAmplification(HealingAmplification::Negative), BonusType::Artifact, 30f32, source!(EminenceOfWinter), None),
            Bonus::new(Attribute::HealingAmplification(HealingAmplification::Repair), BonusType::Artifact, 30f32, source!(EminenceOfWinter), None)
        ]
        4f32 => vec![
            Bonus::new(Attribute::Health(Health::Scalar),BonusType::Legendary, 0.1f32, source!(EminenceOfWinter), None),
        ]
        5f32 => vec![
            Bonus::new(Attribute::ThreatMultipler(Threat::All), BonusType::Artifact, 1f32, source!(EminenceOfWinter), None)
        ]
        6f32 => vec![
            Bonus::new(Attribute::AbilityScore(Ability::Constitution), BonusType::Artifact, 4f32, source!(EminenceOfWinter), None),
        ]
        7f32 => vec![
            Bonus::new(Attribute::ArmorClass(ArmorClass::Scalar), BonusType::Artifact, 0.15f32, source!(EminenceOfWinter), None),
        ]
    )
);

// TODO: Slave Lords Sets
// TODO: Pain and Suffering
// TODO: One with the Swarm
