//! Application Starting Point

use builder::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType},
    breakdowns::Breakdowns,
    types::ability::Ability,
};
use data::ImportSetBonuses;
use ron::ser::PrettyConfig;

fn main() {
    let mut breakdowns = Breakdowns::new();

    breakdowns.import_set_bonuses();

    breakdowns.insert_bonus(Bonus::new(
        Attribute::SetBonus("Might of the Abashai".to_string()),
        BonusType::Stacking,
        5,
        BonusSource::Custom(0),
        None,
    ));
    // clean up

    breakdowns.insert_bonus(Bonus::new(
        Ability::All,
        BonusType::Stacking,
        50,
        BonusSource::Custom(1),
        None,
    ));

    println!(
        "{}",
        ron::ser::to_string_pretty(&breakdowns, PrettyConfig::new()).unwrap()
    );
}
