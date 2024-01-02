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
    fn all_maps_to_all_abilities() {
        let mut compiler = Breakdowns::new();
        compiler.insert_bonus(Bonus::new(
            Attribute::Ability(Ability::All),
            BonusType::Stacking,
            2,
            BonusSource::Debug(0),
            None,
        ));
        for ability in Ability::ABILITIES {
            assert!(compiler.get_attribute(&Attribute::Ability(ability)) == 10.into());
        }
    }
}
