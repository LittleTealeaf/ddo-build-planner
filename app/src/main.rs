//! Application Starting Point

use std::ops::Not;

use builder::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType, Condition, ToValue, Value},
    breakdowns::Breakdowns,
    types::{
        absorption::{Absorption, AbsorptionSource},
        damage_type::DamageType,
    },
};
use data::IncludeSetBonuses;
use ron::{
    ser::{to_string_pretty, PrettyConfig},
    to_string,
};
use rust_decimal::Decimal;

fn main() {
    let value = Value::dice(3, 5)
        + Value::from(5) * (Attribute::SpellResistance.to_value())
            / Value::condition(
                Value::from(3).greater_or_equal_to(Value::Attribute(Attribute::ArmorCheckPenalty))
                    & Condition::Constant(true)
                    ^ Value::from(Attribute::SpellPenetration)
                        .less_than(Value::from(3).floor().abs())
                        .not(),
                Value::from(3),
                Value::from(2),
            );

    println!("{}", to_string(&value).unwrap());

    // let mut breakdowns = Breakdowns::new();
    // breakdowns.import_set_bonuses().unwrap();
    //
    // breakdowns.insert_bonus(Bonus::new(
    //     Absorption::Bonus(DamageType::Fire, AbsorptionSource::EnergySheathe),
    //     BonusType::Standard,
    //     Decimal::try_from(0.5).unwrap(),
    //     BonusSource::Custom(0),
    //     None,
    // ));
    //
    // println!(
    //     "{}",
    //     to_string_pretty(&breakdowns, PrettyConfig::new()).unwrap()
    // );
}
