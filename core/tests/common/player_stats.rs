use ddo_core::{
    bonus::{Bonus, BonusSource, BonusType},
    player_stats::PlayerStats,
    types::ability::Ability,
};
use rust_decimal::Decimal;

#[macro_export]
macro_rules! provider {
    () => {
        ddo_core::types::bonus_provider::BonusProvider::Debug(format!("{}{}", file!(), line!()))
    };
    ($val: expr) => {
        ddo_core::types::bonus_provider::BonusProvider::Debug(format!(
            "{}{}{}",
            file!(),
            line!(),
            $val
        ))
    };
}

pub fn set_ability_score(stats: &mut PlayerStats, ability: Ability, value: Decimal) {
    let provider = provider!();
    stats.clear_provider(&provider);
    let current = stats.evaluate_attribute(ability.score(), None);
    stats.insert_bonus(
        Bonus::new(
            ability.score(),
            value - current,
            BonusType::Stacking,
            BonusSource::Debug(0),
        ),
        &provider,
        None,
    );
}

#[test]
fn test_set_ability_scores() {
    let mut stats = PlayerStats::default();
    for i in [5, 10, 20, 30, 20, 15, 35, 32, 43, 84, 23, 103, 9] {
        set_ability_score(&mut stats, Ability::Charisma, Decimal::from(i));
        assert_eq!(
            Decimal::from(i),
            stats.evaluate_attribute(Ability::Charisma.score(), None)
        );
    }
}
