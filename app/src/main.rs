//! Application Starting Point

use builder::breakdowns::Breakdowns;
use data::IncludeSetBonuses;

fn main() {
    let mut breakdowns = Breakdowns::new();
    breakdowns.import_set_bonuses().unwrap();
}
