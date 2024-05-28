//! Application Starting Point

use core::iter::once;

use builder::{
    attribute::Attribute,
    bonus::{BonusSource, BonusTemplate, BonusType},
    breakdowns::Breakdowns,
    feat::{HeroicPastLife, IconicPastLife, RacialPastLife},
    types::{
        race::Race,
        toggle::{GuildAmenity, Toggle},
    },
};
use ron::ser::{to_string_pretty, PrettyConfig};
use utils::{chain_tree, enums::StaticOptions};

fn main() {
    let mut breakdowns = Breakdowns::new();

    for attribute in Attribute::get_static() {
        breakdowns.track_breakdown(attribute);
    }

    breakdowns.insert_bonuses(
        chain_tree!(
            once(BonusTemplate::new(
                Attribute::GuildLevel,
                BonusType::Stacking,
                200
            )),
            IconicPastLife::get_static().map(BonusTemplate::feat),
            once(BonusTemplate::new(
                Toggle::IconicPastLife(IconicPastLife(Race::Razorclaw)),
                BonusType::Stacking,
                1,
            )),
            HeroicPastLife::get_static().map(BonusTemplate::feat),
            RacialPastLife::get_static().map(BonusTemplate::feat),
            once(BonusTemplate::new(
                Attribute::GuildLevel,
                BonusType::Stacking,
                200
            )),
            GuildAmenity::ALL
                .into_iter()
                .map(|ga| { BonusTemplate::new(Toggle::Guild(ga), BonusType::Standard, 1,) }),
        )
        .map(|bonus| bonus.to_bonus(BonusSource::Debug(1))),
    );

    println!(
        "{}",
        to_string_pretty(&breakdowns, PrettyConfig::new()).unwrap()
    );
}
