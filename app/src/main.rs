//! Application Starting Point

use core::ops::Not;

use builder::{
    attribute::Attribute,
    bonus::{Condition, ToValue, Value},
    breakdowns::Breakdowns,
};
use ron::to_string;

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

    let mut breakdowns = Breakdowns::new();

    println!("{}", breakdowns.evaluate_value(&value));

    println!("{}", to_string(&value).unwrap());
}
