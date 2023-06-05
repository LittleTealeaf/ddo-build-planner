// This tests the compiler with a large set of bonuses

use builder::{
    attribute::types::Ability,
    bonus::{Bonus, BonusSource, BonusType},
    compiler::Compiler,
};

#[test]
fn test_first_life_healer() {
    let mut compiler = Compiler::default();

    // +8 Stat Tome
    compiler.add_bonus(Bonus::new(
        Ability::All.into(),
        BonusType::Stacking,
        8f32.into(),
        BonusSource::Custom(0),
        None,
    ));

    // Ability Bonuses
    compiler.add_bonuses(vec![
        Bonus::new(
            Ability::Constitution.into(),
            BonusType::Stacking,
            10f32.into(),
            BonusSource::Custom(1),
            None,
        ),
        Bonus::new(
            Ability::Wisdom.into(),
            BonusType::Stacking,
            6f32.into(),
            BonusSource::Custom(1),
            None,
        ),
        Bonus::new(
            Ability::Charisma.into(),
            BonusType::Stacking,
            8f32.into(),
            BonusSource::Custom(1),
            None,
        ),
    ]);

    // Level Ups
    compiler.add_bonus(Bonus::new(
        Ability::Constitution.into(),
        BonusType::Stacking,
        8f32.into(),
        BonusSource::Custom(2),
        None,
    ));
}
