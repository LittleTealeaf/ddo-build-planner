use std::collections::HashMap;

use ddo_core::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType},
    player_stats::PlayerStats,
    traits::IterValues,
    types::{ability::Ability, bonus_provider::BonusProvider, player_class::PlayerClass},
    val,
};

pub fn main() {
    env_logger::init();
    let mut stats = PlayerStats::default();
    stats.insert_bonuses(
        [
            Bonus::new(
                Ability::Constitution.score(),
                val!(12),
                BonusType::Standard,
                BonusSource::Debug(0),
            ),
            Bonus::new(
                PlayerClass::Barbarian.level(),
                val!(12),
                BonusType::Standard,
                BonusSource::Debug(0),
            ),
        ],
        BonusProvider::Debug("Hello".to_owned()),
        None,
    );
    let mut values = HashMap::new();
    for attr in Attribute::values() {
        values.insert(attr.clone(), stats.evaluate_attribute(attr, None));
    }
    for (attribute, value) in values {
        println!("{attribute}: {value}");
    }
}
