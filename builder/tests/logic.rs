// Tests that revolve around testing properly implemented logic

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
            assert!(Breakdowns::new().get_attribute(&Attribute::Ability(ability)) == 8.into());
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
                assert!(compiler.get_attribute(&Attribute::AbilityModifier(ability)) == *modifier);
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
            let result_value = breakdowns.get_attribute(&Attribute::Ability(ability));
            assert_eq!(Decimal::from(18), result_value);
        }
    }
}

mod saving_throw {
    use builder::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType},
        breakdowns::Breakdowns,
        types::{ability::Ability, saving_throw::SavingThrow},
    };

    #[test]
    fn dexterity_increases_reflex() {
        let mut breakdowns = Breakdowns::new();
        let initial = breakdowns.get_attribute(&Attribute::SavingThrow(SavingThrow::Reflex));
        breakdowns.insert_bonus(Bonus::new(
            Attribute::Ability(Ability::Dexterity),
            BonusType::Stacking,
            10,
            BonusSource::Debug(0),
            None,
        ));
        let result = breakdowns.get_attribute(&Attribute::SavingThrow(SavingThrow::Reflex));
        assert_eq!(result - initial, 5.into());
    }

    #[test]
    fn wisdom_increases_will() {
        let mut breakdowns = Breakdowns::new();
        let initial = breakdowns.get_attribute(&Attribute::SavingThrow(SavingThrow::Will));
        breakdowns.insert_bonus(Bonus::new(
            Attribute::Ability(Ability::Wisdom),
            BonusType::Stacking,
            10,
            BonusSource::Debug(0),
            None,
        ));
        let result = breakdowns.get_attribute(&Attribute::SavingThrow(SavingThrow::Will));
        assert_eq!(result - initial, 5.into());
    }

    #[test]
    fn constitution_increases_fort() {
        let mut breakdowns = Breakdowns::new();
        let initial = breakdowns.get_attribute(&Attribute::SavingThrow(SavingThrow::Fortitude));
        breakdowns.insert_bonus(Bonus::new(
            Attribute::Ability(Ability::Constitution),
            BonusType::Stacking,
            10,
            BonusSource::Debug(0),
            None,
        ));
        let result = breakdowns.get_attribute(&Attribute::SavingThrow(SavingThrow::Fortitude));
        assert_eq!(result - initial, 5.into());
    }
}

mod skills {
    use builder::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType},
        breakdowns::Breakdowns,
        types::{ability::Ability, skill::Skill},
    };
    use rust_decimal::Decimal;

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
            assert_eq!(
                breakdowns.get_attribute(&Attribute::from(skill)),
                Decimal::ZERO
            );
        }
    }

    #[test]
    fn ability_increases_skill() {
        let pairs = [
            (Ability::Dexterity, Skill::Balance),
            (Ability::Charisma, Skill::Bluff),
            (Ability::Constitution, Skill::Concentration),
            (Ability::Charisma, Skill::Diplomacy),
            (Ability::Intelligence, Skill::DisableDevice),
            (Ability::Charisma, Skill::Haggle),
            (Ability::Wisdom, Skill::Heal),
            (Ability::Dexterity, Skill::Hide),
            (Ability::Charisma, Skill::Intimidate),
            (Ability::Strength, Skill::Jump),
            (Ability::Wisdom, Skill::Listen),
            (Ability::Dexterity, Skill::MoveSilently),
            (Ability::Dexterity, Skill::OpenLock),
            (Ability::Charisma, Skill::Perform),
            (Ability::Intelligence, Skill::Repair),
            (Ability::Intelligence, Skill::Search),
            (Ability::Intelligence, Skill::Spellcraft),
            (Ability::Wisdom, Skill::Spot),
            (Ability::Strength, Skill::Swim),
            (Ability::Dexterity, Skill::Tumble),
            (Ability::Charisma, Skill::UseMagicalDevice),
        ];

        for (ability, skill) in pairs {
            let mut breakdowns = Breakdowns::new();

            let value = breakdowns.get_attribute(&Attribute::from(skill));
            assert_eq!(value, Decimal::from(-1));

            breakdowns.insert_bonus(Bonus::new(
                ability,
                BonusType::Stacking,
                4,
                BonusSource::Debug(0),
                None,
            ));
            let result_value = breakdowns.get_attribute(&Attribute::from(skill));

            assert_eq!(result_value, Decimal::from(1));
        }
    }
}

mod spellcasting {
    mod spell_power {
        use builder::{
            attribute::Attribute,
            bonus::{Bonus, BonusSource, BonusType},
            breakdowns::Breakdowns,
            types::{damage_type::DamageType, skill::Skill, spell_power::SpellPower},
        };
        use rust_decimal::Decimal;

        #[test]
        fn skill_increases_spell_power() {
            let pairs = [
                (Skill::Heal, DamageType::Positive),
                (Skill::Heal, DamageType::Negative),
                (Skill::Perform, DamageType::Sonic),
                (Skill::Spellcraft, DamageType::Acid),
                (Skill::Spellcraft, DamageType::Cold),
                (Skill::Spellcraft, DamageType::Electric),
                (Skill::Spellcraft, DamageType::Fire),
                (Skill::Spellcraft, DamageType::Force),
                (Skill::Spellcraft, DamageType::Light),
                (Skill::Spellcraft, DamageType::Poison),
            ];

            for (skill, damage) in pairs {
                let mut breakdowns = Breakdowns::new();
                breakdowns.insert_bonus(Bonus::new(
                    skill,
                    BonusType::Stacking,
                    2,
                    BonusSource::Debug(0),
                    None,
                ));
                let result =
                    breakdowns.get_attribute(&Attribute::SpellPower(SpellPower::from(damage)));
                assert_eq!(result, Decimal::from(1));
            }
        }

        #[test]
        fn universal_increases_others() {
            for sp in SpellPower::SPELL_POWERS {
                let mut breakdowns = Breakdowns::new();

                let initial = breakdowns.get_attribute(&Attribute::SpellPower(sp));

                breakdowns.insert_bonus(Bonus::new(
                    Attribute::SpellPower(SpellPower::Universal),
                    BonusType::Stacking,
                    100,
                    BonusSource::Debug(0),
                    None,
                ));

                let result = breakdowns.get_attribute(&Attribute::SpellPower(sp));

                assert_eq!(result - initial, 100.into());
            }
        }
    }

    mod critical_chance {
        use builder::{
            attribute::Attribute,
            bonus::{Bonus, BonusSource, BonusType},
            breakdowns::Breakdowns,
            types::spell_power::SpellPower,
        };

        #[test]
        fn universal_increases_others() {
            for sp in SpellPower::SPELL_POWERS {
                let mut breakdowns = Breakdowns::new();

                let initial = breakdowns.get_attribute(&Attribute::SpellCriticalChance(sp));

                breakdowns.insert_bonus(Bonus::new(
                    Attribute::SpellCriticalChance(SpellPower::Universal),
                    BonusType::Stacking,
                    100,
                    BonusSource::Debug(0),
                    None,
                ));

                let result = breakdowns.get_attribute(&Attribute::SpellCriticalChance(sp));

                assert_eq!(result - initial, 100.into());
            }
        }
    }
    mod critical_damage {
        use builder::{
            attribute::Attribute,
            bonus::{Bonus, BonusSource, BonusType},
            breakdowns::Breakdowns,
            types::spell_power::SpellPower,
        };

        #[test]
        fn universal_increases_others() {
            for sp in SpellPower::SPELL_POWERS {
                let mut breakdowns = Breakdowns::new();

                let initial = breakdowns.get_attribute(&Attribute::SpellCriticalDamage(sp));

                breakdowns.insert_bonus(Bonus::new(
                    Attribute::SpellCriticalDamage(SpellPower::Universal),
                    BonusType::Stacking,
                    100,
                    BonusSource::Debug(0),
                    None,
                ));

                let result = breakdowns.get_attribute(&Attribute::SpellCriticalDamage(sp));

                assert_eq!(result - initial, 100.into());
            }
        }
    }
}

mod sheltering {

    mod reduction {
        use builder::{
            attribute::Attribute,
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
                    Attribute::Sheltering(Sheltering::Physical),
                    BonusType::Stacking,
                    input,
                    BonusSource::Debug(0),
                    None,
                ));
                assert_eq!(
                    breakdowns.get_attribute(&Attribute::Sheltering(Sheltering::PhysicalReduction)),
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
                        Attribute::Sheltering(Sheltering::MagicalCap),
                        BonusType::Stacking,
                        1000,
                        BonusSource::Debug(0),
                        None,
                    ),
                    Bonus::new(
                        Attribute::Sheltering(Sheltering::Magical),
                        BonusType::Stacking,
                        input,
                        BonusSource::Debug(0),
                        None,
                    ),
                ]);
                assert_eq!(
                    breakdowns.get_attribute(&Attribute::Sheltering(Sheltering::MagicalReduction)),
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
                breakdowns.get_attribute(&Attribute::Sheltering(Sheltering::MagicalCap)),
                Decimal::from(50)
            );

            breakdowns.insert_bonus(Bonus::new(
                Attribute::Sheltering(Sheltering::Magical),
                BonusType::Stacking,
                75,
                BonusSource::Debug(0),
                None,
            ));

            assert_eq!(
                breakdowns.get_attribute(&Attribute::Sheltering(Sheltering::MagicalTotal)),
                Decimal::from(50)
            );

            breakdowns.insert_bonus(Bonus::flag(ArmorType::Cloth, BonusSource::Debug(1)));

            assert_eq!(
                breakdowns.get_attribute(&Attribute::Sheltering(Sheltering::MagicalTotal)),
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

            assert_eq!(breakdowns.get_attribute(&CAP), 100.into());
            assert_eq!(breakdowns.get_attribute(&TOTAL), 0.into());

            breakdowns.insert_bonus(Bonus::new(
                BONUSES,
                BonusType::Stacking,
                125,
                BonusSource::Debug(1),
                None,
            ));

            assert_eq!(breakdowns.get_attribute(&CAP), 100.into());
            assert_eq!(breakdowns.get_attribute(&TOTAL), 100.into());
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
                        BONUSES,
                        BonusType::Stacking,
                        200,
                        BonusSource::Debug(0),
                        None,
                    ),
                ]);

                assert_eq!(breakdowns.get_attribute(&TOTAL), 200.into());
            }
        }
    }
}
