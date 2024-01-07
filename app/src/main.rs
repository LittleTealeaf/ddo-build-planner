//! Application Starting Point

use builder::{
    bonus::{Bonus, BonusSource, BonusType},
    breakdowns::Breakdowns,
    types::{ability::Ability, sheltering::Sheltering},
};

fn main() {
    let mut breakdowns = Breakdowns::new();
    breakdowns.insert_bonuses([
        Bonus::new(
            Sheltering::Physical,
            BonusType::Stacking,
            100,
            BonusSource::Custom(0),
            None,
        ),
        Bonus::new(
            Ability::All,
            BonusType::Stacking,
            30,
            BonusSource::Custom(0),
            None,
        ),
    ]);
    println!("{breakdowns:?}");
}
