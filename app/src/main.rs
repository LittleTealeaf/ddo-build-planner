//! Application Starting Point

use builder::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType},
    breakdowns::Breakdowns,
    types::toggle::{GuildAmenity, Toggle},
};
use ron::ser::{to_string_pretty, PrettyConfig};

fn main() {
    let mut breakdowns = Breakdowns::new();
    breakdowns.insert_bonus(Bonus::new(
        Attribute::GuildLevel,
        BonusType::Stacking,
        200,
        BonusSource::Debug(0),
    ));

    breakdowns.insert_bonuses(GuildAmenity::ALL.into_iter().map(|ga| {
        Bonus::new(
            Toggle::Guild(ga),
            BonusType::Standard,
            1,
            BonusSource::Debug(2),
        )
    }));

    println!(
        "{}",
        to_string_pretty(&breakdowns, PrettyConfig::new()).unwrap()
    );
}
