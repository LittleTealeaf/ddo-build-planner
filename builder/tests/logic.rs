// Tests that revolve around testing properly implemented logic

mod ability {
    use builder::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType},
        types::ability::Ability, breakdowns::Breakdowns,
    };
    use utils::float::ErrorMargin;

    #[test]
    fn base_score_is_8() {
        for ability in Ability::ABILITIES {
            assert!(Breakdowns::new()
                .get_attribute(&Attribute::Ability(ability))
                .within_margin(&8f32));
        }
    }

    #[test]
    fn modifier_calculates_correctly() {
        let values = [
            (6f32, -2f32),
            (7f32, -2f32),
            (8f32, -1f32),
            (9f32, -1f32),
            (10f32, 0f32),
            (11f32, 0f32),
            (12f32, 1f32),
            (13f32, 1f32),
            (14f32, 2f32),
            (15f32, 2f32),
            (16f32, 3f32),
            (17f32, 3f32),
            (18f32, 4f32),
            (19f32, 4f32),
            (20f32, 5f32),
        ];
        for ability in Ability::ABILITIES {
            for (score, modifier) in &values {
                let mut compiler = Breakdowns::new();
                compiler.add_bonus(Bonus::new(
                    Attribute::Ability(ability),
                    BonusType::Stacking,
                    (score - 8f32).into(),
                    BonusSource::Debug(0),
                    None,
                ));
                assert!(compiler
                    .get_attribute(&Attribute::AbilityModifier(ability))
                    .within_margin(modifier));
            }
        }
    }

    #[test]
    fn all_maps_to_all_abilities() {
        let mut compiler = Breakdowns::new();
        compiler.add_bonus(Bonus::new(
            Attribute::Ability(Ability::All),
            BonusType::Stacking,
            2f32.into(),
            BonusSource::Debug(0),
            None,
        ));
        for ability in Ability::ABILITIES {
            assert!(compiler
                .get_attribute(&Attribute::Ability(ability))
                .within_margin(&10f32));
        }
    }
}
