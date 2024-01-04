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
            let result = breakdowns.get_attribute(&Attribute::SpellPower(SpellPower::from(damage)));
            assert_eq!(result, Decimal::from(1));
        }
    }
}
