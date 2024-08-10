//! Application Starting Point

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
use utils::{chain_tree, enums::StaticValues};

fn main() {
    let mut breakdowns = Breakdowns::new();

    for attribute in Attribute::values() {
        breakdowns.add_breakdown(attribute);
    }

    breakdowns.insert_bonuses(
        chain_tree!(
            [
                BonusTemplate::new(Attribute::GuildLevel, BonusType::Stacking, 200),
                BonusTemplate::new(
                    Toggle::IconicPastLife(IconicPastLife(Race::Razorclaw)),
                    BonusType::Stacking,
                    1,
                ),
                BonusTemplate::new(Attribute::GuildLevel, BonusType::Stacking, 200)
            ],
            IconicPastLife::values().map(BonusTemplate::feat),
            HeroicPastLife::values().map(BonusTemplate::feat),
            RacialPastLife::values().map(BonusTemplate::feat),
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
