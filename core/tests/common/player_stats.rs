use ddo_core::{
    bonus::{Bonus, BonusSource, BonusType},
    player_stats::PlayerStats,
    types::ability::Ability,
};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

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
    println!(
        "Found Score of {current} and Modifier of {}",
        stats.evaluate_attribute(ability.modifier(), None)
    );

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

    let current = stats.evaluate_attribute(ability.score(), None);
    let current_modifier = stats.evaluate_attribute(ability.modifier(), None);
    println!("Finished Score of {current} and Modifier of {current_modifier}");
}

pub fn set_ability_modifier(stats: &mut PlayerStats, ability: Ability, value: Decimal) {
    set_ability_score(stats, ability, (value + dec!(5)) * dec!(2));
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

#[test]
fn test_set_ability_modifier() {
    let mut stats = PlayerStats::default();
    for i in -4..=10 {
        println!("Testing {i}");
        let val = Decimal::from(i);
        set_ability_modifier(&mut stats, Ability::Constitution, val);
        assert_eq!(
            val,
            stats.evaluate_attribute(Ability::Constitution.modifier(), None)
        );
    }
}
