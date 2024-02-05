//! Application Starting Point

use builder::{
    bonus::{Bonus, BonusSource, BonusType},
    breakdowns::Breakdowns,
    types::{
        absorption::{Absorption, AbsorptionSource},
        damage_type::DamageType,
    },
};
use data::IncludeSetBonuses;
use ron::ser::{to_string_pretty, PrettyConfig};
use rust_decimal::Decimal;

fn main() {
    let mut breakdowns = Breakdowns::new();
    breakdowns.import_set_bonuses().unwrap();

    breakdowns.insert_bonus(Bonus::new(
        Absorption::Bonus(DamageType::Fire, AbsorptionSource::EnergySheathe),
        BonusType::Standard,
        Decimal::try_from(0.5).unwrap(),
        BonusSource::Custom(0),
        None,
    ));

    println!(
        "{}",
        to_string_pretty(&breakdowns, PrettyConfig::new()).unwrap()
    );
}
