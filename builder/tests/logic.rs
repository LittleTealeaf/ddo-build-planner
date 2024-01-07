// Tests that revolve around testing properly implemented logic
// This does not test the actual content of the game, but rather the universal logic things.
// Basically, testing logic that should apply to basically all characters

mod ability {
    use builder::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType},
        breakdowns::Breakdowns,
        types::ability::Ability,
    };
    use rust_decimal::Decimal;

    #[test]
    fn base_score_is_8() {
        for ability in Ability::ABILITIES {
            assert_eq!(Breakdowns::new().get_attribute(ability), 8.into());
        }
    }

    #[test]
    fn modifier_calculates_correctly() {
        let values = [
            (6.into(), (-2).into()),
            (7.into(), (-2).into()),
            (8.into(), (-1).into()),
            (9.into(), (-1).into()),
            (10.into(), 0.into()),
            (11.into(), 0.into()),
            (12.into(), 1.into()),
            (13.into(), 1.into()),
            (14.into(), 2.into()),
            (15.into(), 2.into()),
            (16.into(), 3.into()),
            (17.into(), 3.into()),
            (18.into(), 4.into()),
            (19.into(), 4.into()),
            (20.into(), 5.into()),
        ];
        for ability in Ability::ABILITIES {
            for (score, modifier) in &values {
                let mut compiler = Breakdowns::new();
                compiler.insert_bonus(Bonus::new(
                    Attribute::Ability(ability),
                    BonusType::Stacking,
                    score - Decimal::from(8),
                    BonusSource::Debug(0),
                    None,
                ));
                assert_eq!(
                    compiler.get_attribute(Attribute::AbilityModifier(ability)),
                    *modifier
                );
            }
        }
    }

    #[test]
    fn all_increases_ability_score() {
        for ability in Ability::ABILITIES {
            let mut breakdowns = Breakdowns::new();

            breakdowns.insert_bonus(Bonus::new(
                Attribute::Ability(Ability::All),
                BonusType::Stacking,
                10,
                BonusSource::Debug(0),
                None,
            ));
            let result_value = breakdowns.get_attribute(ability);
            assert_eq!(Decimal::from(18), result_value);
        }
    }
}

mod saving_throw {

    mod ability {
        use builder::{
            attribute::Attribute,
            bonus::{Bonus, BonusSource, BonusType},
            breakdowns::Breakdowns,
            types::{ability::Ability, saving_throw::SavingThrow},
        };

        macro_rules! ability_test {
            ($name: ident, $ability: ident, $save: ident) => {
                #[test]
                fn $name() {
                    let mut breakdowns = Breakdowns::new();
                    let initial = breakdowns.get_attribute(SavingThrow::$save);
                    breakdowns.insert_bonus(Bonus::new(
                        Attribute::Ability(Ability::$ability),
                        BonusType::Stacking,
                        10,
                        BonusSource::Debug(0),
                        None,
                    ));
                    let result = breakdowns.get_attribute(SavingThrow::$save);
                    assert_eq!(result - initial, 5.into());
                }
            };
        }

        ability_test!(dexterity_to_reflex, Dexterity, Reflex);
        ability_test!(wisdom_to_will, Wisdom, Will);
        ability_test!(constitution_to_fortitude, Constitution, Fortitude);
    }
}

mod skills {
    use builder::{
        bonus::{Bonus, BonusSource, BonusType},
        breakdowns::Breakdowns,
        types::{ability::Ability, skill::Skill},
    };
    use rust_decimal::Decimal;

    mod ability {
        use super::*;

        macro_rules! ability_test {
            ($name: ident, $ability:ident, $skill:ident) => {
                #[test]
                fn $name() {
                    let mut breakdowns = Breakdowns::new();

                    let value = breakdowns.get_attribute(Skill::$skill);

                    breakdowns.insert_bonus(Bonus::new(
                        Ability::$ability,
                        BonusType::Stacking,
                        4,
                        BonusSource::Debug(0),
                        None,
                    ));
                    let result_value = breakdowns.get_attribute(Skill::$skill);

                    assert_eq!(result_value - value, Decimal::from(2));
                }
            };
        }

        ability_test!(dexterity_to_balance, Dexterity, Balance);
        ability_test!(charisma_to_bluff, Charisma, Bluff);
        ability_test!(constitution_to_concentration, Constitution, Concentration);
        ability_test!(charisma_to_diplomacy, Charisma, Diplomacy);
        ability_test!(intelligence_to_disable_device, Intelligence, DisableDevice);
        ability_test!(charisma_to_haggle, Charisma, Haggle);
        ability_test!(wisdom_to_heal, Wisdom, Heal);
        ability_test!(dexterity_to_hide, Dexterity, Hide);
        ability_test!(charisma_to_intimidate, Charisma, Intimidate);
        ability_test!(strength_to_jump, Strength, Jump);
        ability_test!(wisdom_to_listen, Wisdom, Listen);
        ability_test!(dexterity_to_move_silently, Dexterity, MoveSilently);
        ability_test!(dexterity_to_open_lock, Dexterity, OpenLock);
        ability_test!(charisma_to_perfom, Charisma, Perform);
        ability_test!(intelligence_to_repair, Intelligence, Repair);
        ability_test!(intelligence_to_search, Intelligence, Search);
        ability_test!(intelligence_to_spellcraft, Intelligence, Spellcraft);
        ability_test!(wisdom_to_spot, Wisdom, Spot);
        ability_test!(strength_to_swim, Strength, Swim);
        ability_test!(dexterity_to_tumble, Dexterity, Tumble);
        ability_test!(charisma_to_use_magical_device, Charisma, UseMagicalDevice);
    }

    #[test]
    fn default_no_skill_bonus() {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonus(Bonus::new(
            Ability::All,
            BonusType::Stacking,
            2,
            BonusSource::Debug(0),
            None,
        ));

        for skill in Skill::SKILLS {
            assert_eq!(breakdowns.get_attribute(skill), Decimal::ZERO);
        }
    }

    #[test]
    fn all_includes_all_skills() {
        for skill in Skill::SKILLS {
            let mut breakdowns = Breakdowns::new();
            let initial = breakdowns.get_attribute(skill);
            breakdowns.insert_bonus(Bonus::new(
                Skill::All,
                BonusType::Stacking,
                10,
                BonusSource::Debug(0),
                None,
            ));
            let result = breakdowns.get_attribute(skill);
            assert_eq!(result - initial, 10.into());
        }
    }
}

mod spells {

    macro_rules! universal_to {
        ($attribute: ident, $name: ident, $damage: ident) => {
            #[test]
            fn $name() {
                let mut breakdowns = Breakdowns::new();

                let initial = breakdowns.get_attribute(Attribute::$attribute(SpellPower::Damage(
                    DamageType::$damage,
                )));

                breakdowns.insert_bonus(Bonus::new(
                    Attribute::$attribute(SpellPower::Universal),
                    BonusType::Stacking,
                    100,
                    BonusSource::Debug(0),
                    None,
                ));

                let result = breakdowns.get_attribute(Attribute::$attribute(SpellPower::Damage(
                    DamageType::$damage,
                )));

                assert_eq!(result - initial, 100.into());
            }
        };
    }

    mod spell_power {
        use builder::{
            attribute::Attribute,
            bonus::{Bonus, BonusSource, BonusType},
            breakdowns::Breakdowns,
            types::{damage_type::DamageType, skill::Skill, spell_power::SpellPower},
        };

        mod skill {
            use super::*;

            macro_rules! skill_test {
                ($name: ident, $skill: ident, $damagetype: ident) => {
                    #[test]
                    fn $name() {
                        let mut breakdowns = Breakdowns::new();

                        let initial = breakdowns.get_attribute(Attribute::SpellPower(
                            SpellPower::from(DamageType::$damagetype),
                        ));

                        breakdowns.insert_bonus(Bonus::new(
                            Skill::$skill,
                            BonusType::Stacking,
                            2,
                            BonusSource::Debug(0),
                            None,
                        ));

                        let result = breakdowns.get_attribute(Attribute::SpellPower(
                            SpellPower::from(DamageType::$damagetype),
                        ));

                        assert_eq!(result - initial, 2.into());
                    }
                };
            }

            skill_test!(heal_to_positive, Heal, Positive);
            skill_test!(heal_to_negative, Heal, Negative);
            skill_test!(perform_to_sonic, Perform, Sonic);
            skill_test!(spellcraft_to_acid, Spellcraft, Acid);
            skill_test!(spellcraft_to_cold, Spellcraft, Cold);
            skill_test!(spellcraft_to_electric, Spellcraft, Electric);
            skill_test!(spellcraft_to_fire, Spellcraft, Fire);
            skill_test!(spellcraft_to_force, Spellcraft, Force);
            skill_test!(spellcraft_to_light, Spellcraft, Light);
            skill_test!(spellcraft_to_poison, Spellcraft, Poison);
        }

        mod universal {
            use super::*;

            universal_to!(SpellPower, to_acid, Acid);
            universal_to!(SpellPower, to_fire, Fire);
            universal_to!(SpellPower, to_cold, Cold);
            universal_to!(SpellPower, to_electric, Electric);
            universal_to!(SpellPower, to_positive, Positive);
            universal_to!(SpellPower, to_negative, Negative);
            universal_to!(SpellPower, to_poison, Poison);
            universal_to!(SpellPower, to_repair, Repair);
            universal_to!(SpellPower, to_rust, Rust);
            universal_to!(SpellPower, to_alignment, Alignment);
            universal_to!(SpellPower, to_light, Light);
        }
    }

    mod critical_chance {
        use builder::{
            attribute::Attribute,
            bonus::{Bonus, BonusSource, BonusType},
            breakdowns::Breakdowns,
            types::{damage_type::DamageType, spell_power::SpellPower},
        };

        mod universal {
            use super::*;

            universal_to!(SpellCriticalChance, to_acid, Acid);
            universal_to!(SpellCriticalChance, to_fire, Fire);
            universal_to!(SpellCriticalChance, to_cold, Cold);
            universal_to!(SpellCriticalChance, to_electric, Electric);
            universal_to!(SpellCriticalChance, to_positive, Positive);
            universal_to!(SpellCriticalChance, to_negative, Negative);
            universal_to!(SpellCriticalChance, to_poison, Poison);
            universal_to!(SpellCriticalChance, to_repair, Repair);
            universal_to!(SpellCriticalChance, to_rust, Rust);
            universal_to!(SpellCriticalChance, to_alignment, Alignment);
            universal_to!(SpellCriticalChance, to_light, Light);
        }
    }
    mod critical_damage {
        use builder::{
            attribute::Attribute,
            bonus::{Bonus, BonusSource, BonusType},
            breakdowns::Breakdowns,
            types::{damage_type::DamageType, spell_power::SpellPower},
        };

        mod universal {
            use super::*;

            universal_to!(SpellCriticalDamage, to_acid, Acid);
            universal_to!(SpellCriticalDamage, to_fire, Fire);
            universal_to!(SpellCriticalDamage, to_cold, Cold);
            universal_to!(SpellCriticalDamage, to_electric, Electric);
            universal_to!(SpellCriticalDamage, to_positive, Positive);
            universal_to!(SpellCriticalDamage, to_negative, Negative);
            universal_to!(SpellCriticalDamage, to_poison, Poison);
            universal_to!(SpellCriticalDamage, to_repair, Repair);
            universal_to!(SpellCriticalDamage, to_rust, Rust);
            universal_to!(SpellCriticalDamage, to_alignment, Alignment);
            universal_to!(SpellCriticalDamage, to_light, Light);
        }
    }
}

mod sheltering {

    mod reduction {
        use builder::{
            bonus::{Bonus, BonusSource, BonusType},
            breakdowns::Breakdowns,
            types::sheltering::Sheltering,
        };
        use rust_decimal::Decimal;

        fn resistance_scale(
            inputs: impl IntoIterator<Item = impl Into<Decimal> + Copy>,
        ) -> impl Iterator<Item = (Decimal, Decimal)> {
            inputs.into_iter().map(|input| {
                (
                    input.into(),
                    Decimal::ONE_HUNDRED
                        * (Decimal::ONE
                            - (Decimal::ONE_HUNDRED / (Decimal::ONE_HUNDRED + input.into()))),
                )
            })
        }

        #[test]
        fn physical() {
            let test_points =
                resistance_scale([0, 50, 100, 150, 200, 250, 300, 350, 400, 450, 500]);

            for (input, expected) in test_points {
                let mut breakdowns = Breakdowns::new();
                breakdowns.insert_bonus(Bonus::new(
                    Sheltering::Physical,
                    BonusType::Stacking,
                    input,
                    BonusSource::Debug(0),
                    None,
                ));
                assert_eq!(
                    breakdowns.get_attribute(Sheltering::PhysicalReduction),
                    expected
                );
            }
        }

        #[test]
        fn magical() {
            let test_points =
                resistance_scale([0, 50, 100, 150, 200, 250, 300, 350, 400, 450, 500]);

            for (input, expected) in test_points {
                let mut breakdowns = Breakdowns::new();
                breakdowns.insert_bonuses([
                    Bonus::new(
                        Sheltering::MagicalCap,
                        BonusType::Stacking,
                        1000,
                        BonusSource::Debug(0),
                        None,
                    ),
                    Bonus::new(
                        Sheltering::Magical,
                        BonusType::Stacking,
                        input,
                        BonusSource::Debug(0),
                        None,
                    ),
                ]);
                assert_eq!(
                    breakdowns.get_attribute(Sheltering::MagicalReduction),
                    expected
                );
            }
        }
    }

    mod mrr_cap {
        use builder::{
            attribute::Attribute,
            bonus::{Bonus, BonusSource, BonusType},
            breakdowns::Breakdowns,
            types::{item::ArmorType, sheltering::Sheltering},
        };
        use rust_decimal::Decimal;

        #[test]
        fn capped_at_50_in_cloth() {
            let mut breakdowns = Breakdowns::new();

            assert_eq!(
                breakdowns.get_attribute(Attribute::Sheltering(Sheltering::MagicalCap)),
                Decimal::from(50)
            );

            breakdowns.insert_bonus(Bonus::new(
                Sheltering::Magical,
                BonusType::Stacking,
                75,
                BonusSource::Debug(0),
                None,
            ));

            assert_eq!(
                breakdowns.get_attribute(Attribute::Sheltering(Sheltering::MagicalTotal)),
                Decimal::from(50)
            );

            breakdowns.insert_bonus(Bonus::flag(ArmorType::Cloth, BonusSource::Debug(1)));

            assert_eq!(
                breakdowns.get_attribute(Attribute::Sheltering(Sheltering::MagicalTotal)),
                Decimal::from(50)
            );
        }

        #[test]
        fn capped_at_100_in_light() {
            const BONUSES: Attribute = Attribute::Sheltering(Sheltering::Magical);
            const CAP: Attribute = Attribute::Sheltering(Sheltering::MagicalCap);
            const TOTAL: Attribute = Attribute::Sheltering(Sheltering::MagicalTotal);

            let mut breakdowns = Breakdowns::new();

            breakdowns.insert_bonus(Bonus::flag(ArmorType::Light, BonusSource::Debug(0)));

            assert_eq!(breakdowns.get_attribute(CAP), 100.into());
            assert_eq!(breakdowns.get_attribute(TOTAL), 0.into());

            breakdowns.insert_bonus(Bonus::new(
                Sheltering::Magical,
                BonusType::Stacking,
                125,
                BonusSource::Debug(1),
                None,
            ));

            assert_eq!(breakdowns.get_attribute(CAP), 100.into());
            assert_eq!(breakdowns.get_attribute(TOTAL), 100.into());
        }

        #[test]
        fn uncapped_in_medium_or_heavy() {
            const BONUSES: Attribute = Attribute::Sheltering(Sheltering::Magical);
            const CAP: Attribute = Attribute::Sheltering(Sheltering::MagicalCap);
            const TOTAL: Attribute = Attribute::Sheltering(Sheltering::MagicalTotal);

            for armor_type in [ArmorType::Medium, ArmorType::Heavy] {
                let mut breakdowns = Breakdowns::new();

                breakdowns.insert_bonuses([
                    Bonus::flag(armor_type, BonusSource::Debug(0)),
                    Bonus::new(
                        Sheltering::Magical,
                        BonusType::Stacking,
                        200,
                        BonusSource::Debug(0),
                        None,
                    ),
                ]);

                assert_eq!(breakdowns.get_attribute(TOTAL), 200.into());
            }
        }
    }
}

mod armor_class {
    use builder::{
        bonus::Bonus,
        breakdowns::Breakdowns,
        debug::DebugValue,
        types::{ability::Ability, armor_class::ArmorClass},
    };

    mod bonuses {
        use super::*;

        macro_rules! attribute_test {
            ($name: ident, $attribute: expr) => {
                #[test]
                fn $name() {
                    let mut breakdowns = Breakdowns::new();
                    let initial = breakdowns.get_attribute(ArmorClass::TotalArmorClass);
                    breakdowns.insert_bonus(Bonus::new(
                        $attribute,
                        DebugValue(0),
                        10,
                        DebugValue(0),
                        None,
                    ));
                    let result = breakdowns.get_attribute(ArmorClass::TotalArmorClass);
                    assert!(result > initial);
                }
            };
        }

        attribute_test!(dexterity, Ability::Dexterity);
        attribute_test!(natural_armor, ArmorClass::NaturalArmor);
        attribute_test!(shield_bonus, ArmorClass::ShieldBonus);
        attribute_test!(armor_bonus, ArmorClass::ArmorBonus);
    }

    mod scalars {
        use super::*;

        #[test]
        fn armor_scalar() {
            let mut breakdowns = Breakdowns::new();
            breakdowns.insert_bonus(Bonus::new(
                ArmorClass::ArmorScalar,
                DebugValue(0),
                1,
                DebugValue(0),
                None,
            ));
            let initial = breakdowns.get_attribute(ArmorClass::TotalArmorClass);
            breakdowns.insert_bonus(Bonus::new(
                ArmorClass::ArmorBonus,
                DebugValue(0),
                10,
                DebugValue(1),
                None,
            ));
            let result = breakdowns.get_attribute(ArmorClass::TotalArmorClass);
            assert_eq!(result - initial, 20.into());
        }

        #[test]
        fn shield_scalar() {
            let mut breakdowns = Breakdowns::new();
            breakdowns.insert_bonus(Bonus::new(
                ArmorClass::ShieldScalar,
                DebugValue(0),
                1,
                DebugValue(0),
                None,
            ));
            let initial = breakdowns.get_attribute(ArmorClass::TotalArmorClass);
            breakdowns.insert_bonus(Bonus::new(
                ArmorClass::ShieldBonus,
                DebugValue(0),
                10,
                DebugValue(1),
                None,
            ));
            let result = breakdowns.get_attribute(ArmorClass::TotalArmorClass);
            assert_eq!(result - initial, 20.into());
        }

        #[test]
        fn total_scalar() {
            let mut breakdowns = Breakdowns::new();
            breakdowns.insert_bonus(Bonus::new(
                ArmorClass::Bonus,
                DebugValue(0),
                100,
                DebugValue(0),
                None,
            ));
            let initial = breakdowns.get_attribute(ArmorClass::TotalArmorClass);
            breakdowns.insert_bonus(Bonus::new(
                ArmorClass::TotalScalar,
                DebugValue(0),
                1,
                DebugValue(1),
                None,
            ));
            let result = breakdowns.get_attribute(ArmorClass::TotalArmorClass);

            assert_eq!(result / initial, 2.into());
        }
    }

    mod max_dex_bonus {
        use builder::types::{
            flag::OffHandType,
            item::{ArmorType, ShieldType},
        };

        use super::*;

        macro_rules! dex_test {
            ($name: ident, $maxbonus: ident, $flag: expr) => {
                #[test]
                fn $name() {
                    let mut breakdowns = Breakdowns::new();

                    breakdowns.insert_bonus(Bonus::new(
                        Ability::Dexterity,
                        DebugValue(0),
                        100,
                        DebugValue(0),
                        None,
                    ));

                    let initial = breakdowns.get_attribute(ArmorClass::TotalArmorClass);

                    breakdowns.insert_bonus(Bonus::flag($flag, DebugValue(1)));

                    let with_armor = breakdowns.get_attribute(ArmorClass::TotalArmorClass);

                    assert!(initial > with_armor);

                    breakdowns.insert_bonus(Bonus::new(
                        ArmorClass::$maxbonus,
                        DebugValue(0),
                        2,
                        DebugValue(2),
                        None,
                    ));

                    let with_increased_max = breakdowns.get_attribute(ArmorClass::TotalArmorClass);

                    assert_eq!(with_increased_max - with_armor, 2.into());
                }
            };
        }

        dex_test!(light_armor, ArmorMaxDex, ArmorType::Light);
        dex_test!(medium_armor, ArmorMaxDex, ArmorType::Medium);
        dex_test!(heavy_armor, ArmorMaxDex, ArmorType::Heavy);
        dex_test!(
            tower_shield,
            ShieldMaxDex,
            OffHandType::from(ShieldType::TowerShield)
        );

        #[test]
        fn lowest_max_dex() {
            for (a, b) in [(2, 1), (1, 2)] {
                let mut breakdowns = Breakdowns::new();
                breakdowns.insert_bonuses([
                    Bonus::flag(OffHandType::from(ShieldType::TowerShield), DebugValue(0)),
                    Bonus::flag(ArmorType::Heavy, DebugValue(0)),
                    Bonus::new(Ability::Dexterity, DebugValue(0), 100, DebugValue(0), None),
                ]);

                let initial = breakdowns.get_attribute(ArmorClass::TotalArmorClass);

                breakdowns.insert_bonuses([
                    Bonus::new(
                        ArmorClass::ShieldMaxDex,
                        DebugValue(0),
                        a,
                        DebugValue(1),
                        None,
                    ),
                    Bonus::new(
                        ArmorClass::ArmorMaxDex,
                        DebugValue(0),
                        b,
                        DebugValue(1),
                        None,
                    ),
                ]);

                let result = breakdowns.get_attribute(ArmorClass::TotalArmorClass);

                assert_eq!(result - initial, 1.into());
            }
        }
    }
}

mod race {
    mod dwarf {
        use builder::{
            bonus::{Bonus, BonusSource},
            breakdowns::Breakdowns,
            feat::{Feat, Proficiency},
            types::{item::WeaponType, race::Race},
        };

        #[test]
        fn dwarven_war_axe() {
            let mut breakdowns = Breakdowns::new();
            breakdowns.insert_bonus(Bonus::flag(Race::Dwarf, BonusSource::Debug(0)));
            assert_eq!(
                breakdowns.get_attribute(Feat::Proficiency(Proficiency::WeaponProficiency(
                    WeaponType::DwarvenWarAxe
                ))),
                0.into()
            );
            breakdowns.insert_bonus(Bonus::feat(
                Proficiency::MartialWeaponProficiency,
                BonusSource::Debug(1),
                None,
            ));
            assert!(
                breakdowns.get_attribute(Feat::Proficiency(Proficiency::WeaponProficiency(
                    WeaponType::DwarvenWarAxe
                ))) > 0.into()
            );
        }
    }
}

mod feats {
    mod proficiencies {
        use builder::{
            attribute::Attribute,
            bonus::{Bonus, BonusSource, BonusType},
            breakdowns::Breakdowns,
            feat::{Feat, Proficiency},
            types::item::WeaponType,
        };

        #[test]
        fn simple_proficiency_provides_proficiencies() {
            let mut compiler = Breakdowns::new();
            compiler.insert_bonus(Bonus::new(
                Attribute::Feat(Feat::Proficiency(Proficiency::SimpleWeaponProficiency)),
                BonusType::Stacking,
                1,
                BonusSource::Debug(0),
                None,
            ));

            assert!(
                compiler.get_attribute(Attribute::Feat(Feat::Proficiency(
                    Proficiency::WeaponProficiency(WeaponType::Dagger)
                ))) > 0.into()
            );
        }

        #[test]
        fn martial_proficiency_provides_proficiencies() {
            let mut compiler = Breakdowns::new();
            compiler.insert_bonus(Bonus::new(
                Attribute::Feat(Feat::Proficiency(Proficiency::MartialWeaponProficiency)),
                BonusType::Stacking,
                1,
                BonusSource::Debug(0),
                None,
            ));

            assert!(
                compiler.get_attribute(Attribute::Feat(Feat::Proficiency(
                    Proficiency::WeaponProficiency(WeaponType::Falchion)
                ))) > 0.into()
            );
        }
    }
}

mod armor_check_penalty {
    use builder::{
        attribute::Attribute, bonus::Bonus, breakdowns::Breakdowns, debug::DebugValue,
        types::skill::Skill,
    };

    mod skills {
        use super::*;

        macro_rules! acp_skill {
            ($name: ident, $skill: ident, $scale: expr) => {
                #[test]
                fn $name() {
                    let mut breakdowns = Breakdowns::new();
                    let initial = breakdowns.get_attribute(Skill::$skill);
                    breakdowns.insert_bonus(Bonus::new(
                        Attribute::ArmorCheckPenalty,
                        DebugValue(0),
                        1,
                        DebugValue(0),
                        None,
                    ));
                    let result = breakdowns.get_attribute(Skill::$skill);
                    assert_eq!(initial - result, $scale.into());
                }
            };
        }

        acp_skill!(balance, Balance, 1);
        acp_skill!(hide, Hide, 1);
        acp_skill!(jump, Jump, 1);
        acp_skill!(move_silently, MoveSilently, 1);
        acp_skill!(swim, Swim, 2);
        acp_skill!(tumble, Tumble, 1);
    }

    #[test]
    fn ignore_negative_values() {
        let mut breakdowns = Breakdowns::new();
        let initial = breakdowns.get_attribute(Skill::Balance);
        breakdowns.insert_bonus(Bonus::new(
            Attribute::ArmorCheckPenalty,
            DebugValue(0),
            -1,
            DebugValue(0),
            None,
        ));
        let result = breakdowns.get_attribute(Skill::Balance);
        assert_eq!(result, initial);
    }
}
