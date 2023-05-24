use builder_core::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType, Condition},
    compiler::AttributeCompiler,
};

mod conditions {

    use super::*;

    #[test]
    fn has() {
        let mut breakdowns = AttributeCompiler::new();

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::PhysicalSheltering(),
            BonusType::Stacking,
            10f32,
            BonusSource::Unique(0),
            Some(vec![Condition::Has(Attribute::MagicalSheltering())]),
        )]);

        assert_eq!(
            0f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            1f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            10f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            -1f32,
            1.into(),
            None,
        )]);

        assert_eq!(
            0f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );
    }

    #[test]
    fn not_have() {
        let mut breakdowns = AttributeCompiler::new();

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::PhysicalSheltering(),
            BonusType::Stacking,
            10f32,
            BonusSource::Unique(0),
            Some(vec![Condition::NotHave(Attribute::MagicalSheltering())]),
        )]);

        assert_eq!(
            10f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            1f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            0f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );
    }

    #[test]
    fn max() {
        let mut breakdowns = AttributeCompiler::new();

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::PhysicalSheltering(),
            BonusType::Stacking,
            10f32,
            BonusSource::Unique(0),
            Some(vec![Condition::Max(Attribute::MagicalSheltering(), 2f32)]),
        )]);

        assert_eq!(
            10f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            1f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            10f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            2f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            10f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            3f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            0f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );
    }

    #[test]
    fn min() {
        let mut breakdowns = AttributeCompiler::new();

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::PhysicalSheltering(),
            BonusType::Stacking,
            10f32,
            BonusSource::Unique(0),
            Some(vec![Condition::Min(Attribute::MagicalSheltering(), 2f32)]),
        )]);

        assert_eq!(
            0f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            1f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            0f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            2f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            10f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            3f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            10f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );
    }

    #[test]
    fn eq() {
        let mut breakdowns = AttributeCompiler::new();

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::PhysicalSheltering(),
            BonusType::Stacking,
            10f32,
            BonusSource::Unique(0),
            Some(vec![Condition::Eq(Attribute::MagicalSheltering(), 2f32)]),
        )]);

        assert_eq!(
            0f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            1f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            0f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            2f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            10f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            3f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            0f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );
    }

    #[test]
    fn not_eq() {
        let mut breakdowns = AttributeCompiler::new();

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::PhysicalSheltering(),
            BonusType::Stacking,
            10f32,
            BonusSource::Unique(0),
            Some(vec![Condition::NotEq(Attribute::MagicalSheltering(), 2f32)]),
        )]);

        assert_eq!(
            10f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            1f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            10f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            2f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            0f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::MagicalSheltering(),
            BonusType::Stacking,
            3f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            10f32,
            breakdowns.get_attribute(&Attribute::PhysicalSheltering())
        );
    }
}
