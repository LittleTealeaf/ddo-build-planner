// Tests that revolve around testing properly implemented logic
// This does not test the actual content of the game, but rather the universal logic things.
// Basically, testing logic that should apply to basically all characters
use builder::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType},
    breakdowns::Breakdowns,
    debug::DebugValue,
    feat::Proficiency,
    types::{
        ability::Ability,
        armor_class::ArmorClass,
        damage_type::DamageType,
        flag::Flag,
        flag::OffHandType,
        item_type::{ArmorType, ShieldType, WeaponType},
        race::Race,
        saving_throw::SavingThrow,
        sheltering::Sheltering,
        skill::Skill,
        spell_power::SpellPower,
        toggle::Toggle,
    },
};
use rust_decimal::Decimal;

mod ability {
    use super::*;

    #[test]
    fn base_score_is_8() {
        for ability in Ability::ABILITIES {
            assert_eq!(
                Breakdowns::new().evaluate_attribute(&ability.into()),
                8.into()
            );
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
                    None,
                    BonusSource::Debug(0),
                ));
                assert_eq!(
                    compiler.evaluate_attribute(&Attribute::AbilityModifier(ability)),
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
                None,
                BonusSource::Debug(0),
            ));
            let result_value = breakdowns.evaluate_attribute_from(ability);
            assert_eq!(Decimal::from(18), result_value);
        }
    }
}

mod saving_throw {
    use super::*;

    mod ability {
        use super::*;

        macro_rules! ability_test {
            ($name: ident, $ability: ident, $save: ident) => {
                #[test]
                fn $name() {
                    let mut breakdowns = Breakdowns::new();
                    let initial = breakdowns.evaluate_attribute(&SavingThrow::$save.into());
                    breakdowns.insert_bonus(Bonus::new(
                        Attribute::Ability(Ability::$ability),
                        BonusType::Stacking,
                        10,
                        None,
                        BonusSource::Debug(0),
                    ));
                    let result = breakdowns.evaluate_attribute_from(SavingThrow::$save);
                    assert_eq!(result - initial, 5.into());
                }
            };
        }

        ability_test!(dexterity_to_reflex, Dexterity, Reflex);
        ability_test!(wisdom_to_will, Wisdom, Will);
        ability_test!(constitution_to_fortitude, Constitution, Fortitude);
    }

    mod secondary {
        use super::*;

        macro_rules! secondary_test {
            ($name: ident, $parent: ident, $save: ident) => {
                #[test]
                fn $name() {
                    let mut breakdowns = Breakdowns::new();
                    let initial = breakdowns.evaluate_attribute_from(SavingThrow::$save);
                    breakdowns.insert_bonus(Bonus::new(
                        SavingThrow::$parent,
                        DebugValue(0),
                        10,
                        None,
                        DebugValue(0),
                    ));
                    let result = breakdowns.evaluate_attribute_from(SavingThrow::$save);
                    assert_eq!(result - initial, 10.into());
                }
            };
        }

        secondary_test!(fortitude_to_poison, Fortitude, Poison);
        secondary_test!(fortitude_to_disease, Fortitude, Disease);
        secondary_test!(reflex_to_traps, Reflex, Traps);
        secondary_test!(reflex_to_spell, Reflex, Spell);
        secondary_test!(reflex_to_magic, Reflex, Magic);
        secondary_test!(will_to_enchantment, Will, Enchantment);
        secondary_test!(will_to_illusion, Will, Illusion);
        secondary_test!(will_to_fear, Will, Fear);
        secondary_test!(will_to_curse, Will, Curse);
    }
}

mod skills {
    use super::*;

    mod ability {
        use super::*;

        macro_rules! ability_test {
            ($name: ident, $ability:ident, $skill:ident) => {
                #[test]
                fn $name() {
                    let mut breakdowns = Breakdowns::new();

                    let value = breakdowns.evaluate_attribute_from(Skill::$skill);

                    breakdowns.insert_bonus(Bonus::new(
                        Ability::$ability,
                        BonusType::Stacking,
                        4,
                        None,
                        BonusSource::Debug(0),
                    ));
                    let result_value = breakdowns.evaluate_attribute_from(Skill::$skill);

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
            None,
            BonusSource::Debug(0),
        ));

        for skill in Skill::SKILLS {
            assert_eq!(breakdowns.evaluate_attribute_from(skill), Decimal::ZERO);
        }
    }

    #[test]
    fn all_includes_all_skills() {
        for skill in Skill::SKILLS {
            let mut breakdowns = Breakdowns::new();
            let initial = breakdowns.evaluate_attribute_from(skill);
            breakdowns.insert_bonus(Bonus::new(
                Skill::All,
                BonusType::Stacking,
                10,
                None,
                BonusSource::Debug(0),
            ));
            let result = breakdowns.evaluate_attribute_from(skill);
            assert_eq!(result - initial, 10.into());
        }
    }
}

mod spells {
    use super::*;

    macro_rules! universal_to {
        ($attribute: ident, $name: ident, $damage: ident) => {
            #[test]
            fn $name() {
                let mut breakdowns = Breakdowns::new();

                let initial = breakdowns.evaluate_attribute_from(Attribute::$attribute(
                    SpellPower::Damage(DamageType::$damage),
                ));

                breakdowns.insert_bonus(Bonus::new(
                    Attribute::$attribute(SpellPower::Universal),
                    BonusType::Stacking,
                    100,
                    None,
                    BonusSource::Debug(0),
                ));

                let result = breakdowns.evaluate_attribute_from(Attribute::$attribute(
                    SpellPower::Damage(DamageType::$damage),
                ));

                assert_eq!(result - initial, 100.into());
            }
        };
    }

    macro_rules! potency_to {
        ($attribute: ident, $name: ident, $damage: ident) => {
            #[test]
            fn $name() {
                const ATTRIBUTE: Attribute =
                    Attribute::$attribute(SpellPower::Damage(DamageType::$damage));
                let mut breakdowns = Breakdowns::new();

                let initial = breakdowns.evaluate_attribute_from(ATTRIBUTE);

                breakdowns.insert_bonus(Bonus::new(
                    Attribute::$attribute(SpellPower::Potency),
                    DebugValue(0),
                    100,
                    None,
                    DebugValue(0),
                ));

                let with_potency = breakdowns.evaluate_attribute_from(ATTRIBUTE);
                assert_eq!(with_potency - initial, 100.into());

                breakdowns.insert_bonus(Bonus::new(
                    ATTRIBUTE,
                    DebugValue(0),
                    50,
                    None,
                    DebugValue(1),
                ));

                let with_lower = breakdowns.evaluate_attribute_from(ATTRIBUTE);
                assert_eq!(with_potency, with_lower);

                breakdowns.insert_bonus(Bonus::new(
                    ATTRIBUTE,
                    DebugValue(0),
                    150,
                    None,
                    DebugValue(2),
                ));

                let with_higher = breakdowns.evaluate_attribute_from(ATTRIBUTE);
                assert_eq!(with_higher - initial, 150.into());
            }
        };
    }

    mod spell_power {
        use super::*;

        mod skill {
            use super::*;

            macro_rules! skill_test {
                ($name: ident, $skill: ident, $damagetype: ident) => {
                    #[test]
                    fn $name() {
                        let mut breakdowns = Breakdowns::new();

                        let initial = breakdowns.evaluate_attribute_from(Attribute::SpellPower(
                            SpellPower::from(DamageType::$damagetype),
                        ));

                        breakdowns.insert_bonus(Bonus::new(
                            Skill::$skill,
                            BonusType::Stacking,
                            2,
                            None,
                            BonusSource::Debug(0),
                        ));

                        let result = breakdowns.evaluate_attribute_from(Attribute::SpellPower(
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

        mod potency {
            use super::*;

            potency_to!(SpellPower, to_acid, Acid);
            potency_to!(SpellPower, to_fire, Fire);
            potency_to!(SpellPower, to_cold, Cold);
            potency_to!(SpellPower, to_electric, Electric);
            potency_to!(SpellPower, to_positive, Positive);
            potency_to!(SpellPower, to_negative, Negative);
            potency_to!(SpellPower, to_poison, Poison);
            potency_to!(SpellPower, to_repair, Repair);
            potency_to!(SpellPower, to_rust, Rust);
            potency_to!(SpellPower, to_alignment, Alignment);
            potency_to!(SpellPower, to_light, Light);
        }
    }

    mod critical_chance {
        use super::*;

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

        mod potency {
            use super::*;

            potency_to!(SpellCriticalChance, to_acid, Acid);
            potency_to!(SpellCriticalChance, to_fire, Fire);
            potency_to!(SpellCriticalChance, to_cold, Cold);
            potency_to!(SpellCriticalChance, to_electric, Electric);
            potency_to!(SpellCriticalChance, to_positive, Positive);
            potency_to!(SpellCriticalChance, to_negative, Negative);
            potency_to!(SpellCriticalChance, to_poison, Poison);
            potency_to!(SpellCriticalChance, to_repair, Repair);
            potency_to!(SpellCriticalChance, to_rust, Rust);
            potency_to!(SpellCriticalChance, to_alignment, Alignment);
            potency_to!(SpellCriticalChance, to_light, Light);
        }
    }
    mod critical_damage {
        use super::*;

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

        mod potency {
            use super::*;

            potency_to!(SpellCriticalDamage, to_acid, Acid);
            potency_to!(SpellCriticalDamage, to_fire, Fire);
            potency_to!(SpellCriticalDamage, to_cold, Cold);
            potency_to!(SpellCriticalDamage, to_electric, Electric);
            potency_to!(SpellCriticalDamage, to_positive, Positive);
            potency_to!(SpellCriticalDamage, to_negative, Negative);
            potency_to!(SpellCriticalDamage, to_poison, Poison);
            potency_to!(SpellCriticalDamage, to_repair, Repair);
            potency_to!(SpellCriticalDamage, to_rust, Rust);
            potency_to!(SpellCriticalDamage, to_alignment, Alignment);
            potency_to!(SpellCriticalDamage, to_light, Light);
        }
    }
}

mod sheltering {
    use super::*;

    mod reduction {
        use super::*;

        fn resistance_scale<I, D>(inputs: I) -> impl Iterator<Item = (Decimal, Decimal)>
        where
            I: IntoIterator<Item = D>,
            D: Into<Decimal> + Copy,
        {
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
                    None,
                    BonusSource::Debug(0),
                ));
                assert_eq!(
                    breakdowns.evaluate_attribute_from(Sheltering::PhysicalReduction),
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
                        None,
                        BonusSource::Debug(0),
                    ),
                    Bonus::new(
                        Sheltering::Magical,
                        BonusType::Stacking,
                        input,
                        None,
                        BonusSource::Debug(0),
                    ),
                ]);
                assert_eq!(
                    breakdowns.evaluate_attribute_from(Sheltering::MagicalReduction),
                    expected
                );
            }
        }
    }

    mod mrr_cap {
        use super::*;

        #[test]
        fn capped_at_50_in_cloth() {
            let mut breakdowns = Breakdowns::new();

            assert_eq!(
                breakdowns.evaluate_attribute_from(Attribute::Sheltering(Sheltering::MagicalCap)),
                Decimal::from(50)
            );

            breakdowns.insert_bonus(Bonus::new(
                Sheltering::Magical,
                BonusType::Stacking,
                75,
                None,
                BonusSource::Debug(0),
            ));

            assert_eq!(
                breakdowns.evaluate_attribute_from(Attribute::Sheltering(Sheltering::MagicalTotal)),
                Decimal::from(50)
            );

            breakdowns.insert_bonus(Bonus::flag(ArmorType::Cloth, None, BonusSource::Debug(1)));

            assert_eq!(
                breakdowns.evaluate_attribute_from(Attribute::Sheltering(Sheltering::MagicalTotal)),
                Decimal::from(50)
            );
        }

        #[test]
        fn capped_at_100_in_light() {
            const BONUSES: Attribute = Attribute::Sheltering(Sheltering::Magical);
            const CAP: Attribute = Attribute::Sheltering(Sheltering::MagicalCap);
            const TOTAL: Attribute = Attribute::Sheltering(Sheltering::MagicalTotal);

            let mut breakdowns = Breakdowns::new();

            breakdowns.insert_bonus(Bonus::flag(ArmorType::Light, None, BonusSource::Debug(0)));

            assert_eq!(breakdowns.evaluate_attribute_from(CAP), 100.into());
            assert_eq!(breakdowns.evaluate_attribute_from(TOTAL), 0.into());

            breakdowns.insert_bonus(Bonus::new(
                Sheltering::Magical,
                BonusType::Stacking,
                125,
                None,
                BonusSource::Debug(1),
            ));

            assert_eq!(breakdowns.evaluate_attribute_from(CAP), 100.into());
            assert_eq!(breakdowns.evaluate_attribute_from(TOTAL), 100.into());
        }

        #[test]
        fn uncapped_in_medium_or_heavy() {
            const BONUSES: Attribute = Attribute::Sheltering(Sheltering::Magical);
            const CAP: Attribute = Attribute::Sheltering(Sheltering::MagicalCap);
            const TOTAL: Attribute = Attribute::Sheltering(Sheltering::MagicalTotal);

            for armor_type in [ArmorType::Medium, ArmorType::Heavy] {
                let mut breakdowns = Breakdowns::new();

                breakdowns.insert_bonuses([
                    Bonus::flag(armor_type, None, BonusSource::Debug(0)),
                    Bonus::new(
                        Sheltering::Magical,
                        BonusType::Stacking,
                        200,
                        None,
                        BonusSource::Debug(0),
                    ),
                ]);

                assert_eq!(breakdowns.evaluate_attribute_from(TOTAL), 200.into());
            }
        }
    }
}

mod armor_class {
    use super::*;

    mod bonuses {
        use super::*;

        macro_rules! attribute_test {
            ($name: ident, $attribute: expr) => {
                #[test]
                fn $name() {
                    let mut breakdowns = Breakdowns::new();
                    let initial = breakdowns.evaluate_attribute_from(ArmorClass::Total);
                    breakdowns.insert_bonus(Bonus::new(
                        $attribute,
                        DebugValue(0),
                        10,
                        None,
                        DebugValue(0),
                    ));
                    let result = breakdowns.evaluate_attribute_from(ArmorClass::Total);
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
                None,
                DebugValue(0),
            ));
            let initial = breakdowns.evaluate_attribute_from(ArmorClass::Total);
            breakdowns.insert_bonus(Bonus::new(
                ArmorClass::ArmorBonus,
                DebugValue(0),
                10,
                None,
                DebugValue(1),
            ));
            let result = breakdowns.evaluate_attribute_from(ArmorClass::Total);
            assert_eq!(result - initial, 20.into());
        }

        #[test]
        fn shield_scalar() {
            let mut breakdowns = Breakdowns::new();
            breakdowns.insert_bonus(Bonus::new(
                ArmorClass::ShieldScalar,
                DebugValue(0),
                1,
                None,
                DebugValue(0),
            ));
            let initial = breakdowns.evaluate_attribute_from(ArmorClass::Total);
            breakdowns.insert_bonus(Bonus::new(
                ArmorClass::ShieldBonus,
                DebugValue(0),
                10,
                None,
                DebugValue(1),
            ));
            let result = breakdowns.evaluate_attribute_from(ArmorClass::Total);
            assert_eq!(result - initial, 20.into());
        }

        #[test]
        fn total_scalar() {
            let mut breakdowns = Breakdowns::new();
            breakdowns.insert_bonus(Bonus::new(
                ArmorClass::Bonus,
                DebugValue(0),
                100,
                None,
                DebugValue(0),
            ));
            let initial = breakdowns.evaluate_attribute_from(ArmorClass::Total);
            breakdowns.insert_bonus(Bonus::new(
                ArmorClass::TotalScalar,
                DebugValue(0),
                1,
                None,
                DebugValue(1),
            ));
            let result = breakdowns.evaluate_attribute_from(ArmorClass::Total);

            assert_eq!(result / initial, 2.into());
        }
    }

    mod max_dex_bonus {
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
                        None,
                        DebugValue(0),
                    ));

                    let initial = breakdowns.evaluate_attribute_from(ArmorClass::Total);

                    breakdowns.insert_bonus(Bonus::flag($flag, None, DebugValue(1)));

                    let with_armor = breakdowns.evaluate_attribute_from(ArmorClass::Total);

                    assert!(initial > with_armor);

                    breakdowns.insert_bonus(Bonus::new(
                        ArmorClass::$maxbonus,
                        DebugValue(0),
                        2,
                        None,
                        DebugValue(2),
                    ));

                    let with_increased_max = breakdowns.evaluate_attribute_from(ArmorClass::Total);

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
                    Bonus::flag(
                        OffHandType::from(ShieldType::TowerShield),
                        None,
                        DebugValue(0),
                    ),
                    Bonus::flag(ArmorType::Heavy, None, DebugValue(0)),
                    Bonus::new(Ability::Dexterity, DebugValue(0), 100, None, DebugValue(0)),
                ]);

                let initial = breakdowns.evaluate_attribute_from(ArmorClass::Total);

                breakdowns.insert_bonuses([
                    Bonus::new(
                        ArmorClass::ShieldMaxDex,
                        DebugValue(0),
                        a,
                        None,
                        DebugValue(1),
                    ),
                    Bonus::new(
                        ArmorClass::ArmorMaxDex,
                        DebugValue(0),
                        b,
                        None,
                        DebugValue(1),
                    ),
                ]);

                let result = breakdowns.evaluate_attribute_from(ArmorClass::Total);

                assert_eq!(result - initial, 1.into());
            }
        }
    }
}

mod race {
    use super::*;

    mod dwarf {
        use super::*;

        #[test]
        fn dwarven_war_axe() {
            let mut breakdowns = Breakdowns::new();
            breakdowns.insert_bonus(Bonus::flag(Race::Dwarf, None, DebugValue(0)));
            assert_eq!(
                breakdowns.evaluate_attribute_from(Proficiency::from(WeaponType::DwarvenWarAxe)),
                0.into()
            );
            breakdowns.insert_bonus(Bonus::feat(
                Proficiency::MartialWeaponProficiency,
                None,
                DebugValue(1),
            ));
            assert!(
                breakdowns.evaluate_attribute_from(Proficiency::from(WeaponType::DwarvenWarAxe))
                    > 0.into()
            );
        }
    }
}

mod feats {
    use super::*;

    mod proficiencies {
        use super::*;

        #[test]
        fn simple_proficiency_provides_proficiencies() {
            let mut compiler = Breakdowns::new();
            compiler.insert_bonus(Bonus::new(
                Proficiency::SimpleWeaponProficiency,
                BonusType::Stacking,
                1,
                None,
                BonusSource::Debug(0),
            ));

            assert!(
                compiler.evaluate_attribute_from(Proficiency::from(WeaponType::Dagger)) > 0.into()
            );
        }

        #[test]
        fn martial_proficiency_provides_proficiencies() {
            let mut compiler = Breakdowns::new();
            compiler.insert_bonus(Bonus::new(
                Proficiency::MartialWeaponProficiency,
                BonusType::Stacking,
                1,
                None,
                BonusSource::Debug(0),
            ));

            assert!(
                compiler.evaluate_attribute_from(Proficiency::from(WeaponType::Falchion))
                    > 0.into()
            );
        }
    }
}

mod armor_check_penalty {
    use super::*;

    mod skills {
        use super::*;

        macro_rules! acp_skill {
            ($name: ident, $skill: ident, $scale: expr) => {
                #[test]
                fn $name() {
                    let mut breakdowns = Breakdowns::new();
                    let initial = breakdowns.evaluate_attribute_from(Skill::$skill);
                    breakdowns.insert_bonus(Bonus::new(
                        Attribute::ArmorCheckPenalty,
                        DebugValue(0),
                        1,
                        None,
                        DebugValue(0),
                    ));
                    let result = breakdowns.evaluate_attribute_from(Skill::$skill);
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
        let initial = breakdowns.evaluate_attribute_from(Skill::Balance);
        breakdowns.insert_bonus(Bonus::new(
            Attribute::ArmorCheckPenalty,
            DebugValue(0),
            -1,
            None,
            DebugValue(0),
        ));
        let result = breakdowns.evaluate_attribute_from(Skill::Balance);
        assert_eq!(result, initial);
    }
}

mod toggles {

    use super::*;

    #[test]
    fn active_togge_provides_use() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonus(Bonus::new(
            Attribute::Toggle(Toggle::Blocking),
            BonusType::Stacking,
            1,
            None,
            DebugValue(0),
        ));

        assert!(breakdowns.evaluate_attribute_from(Flag::HasToggle(Toggle::Blocking)) > 0.into());
    }
}
