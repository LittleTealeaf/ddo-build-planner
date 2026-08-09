use std::collections::HashSet;

use ddo_core::player_stats::PlayerStats;

pub fn main() {
    let mut stats = PlayerStats::default();
    let bonuses = stats.bonuses();
    let mut attributes = HashSet::new();
    for bonus in bonuses {
        println!("{bonus}");
        attributes.insert(bonus.attribute().clone());
    }
    println!("Attributes:");
    for attribute in attributes {
        let val = stats.evaluate_attribute(attribute.clone(), None);
        println!("{attribute}: {val}");
    }
}
