use ddo_core::{
    attribute::Attribute,
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

pub fn set_attribute_value<I: Into<Attribute>>(
    stats: &mut PlayerStats,
    attribute: I,
    value: Decimal,
) {
    let attribute = attribute.into();
    let provider = provider!(format!("{attribute}"));

    stats.clear_provider(&provider);
    let current = stats.evaluate_attribute(attribute.clone(), None);
    let difference = value - current;
    stats.insert_bonus(
        Bonus::new(
            attribute.clone(),
            difference,
            BonusType::Stacking,
            BonusSource::Debug(0),
        ),
        &provider,
        None,
    );
    let result = stats.evaluate_attribute(attribute.clone(), None);
    assert_eq!(
        value, result,
        "Expected Attribute ({attribute}) to be {value}, found {result}"
    );
}

pub fn set_ability_score(stats: &mut PlayerStats, ability: Ability, value: Decimal) {
    set_attribute_value(stats, ability.score(), value);
}

pub fn set_ability_modifier(stats: &mut PlayerStats, ability: Ability, value: Decimal) {
    set_ability_score(stats, ability, (value + dec!(5)) * dec!(2));
    let result = stats.evaluate_attribute(ability.modifier(), None);
    assert_eq!(
        value, result,
        "Expected {ability} Ability Modifier to be {value}, found {result}"
    );
}
