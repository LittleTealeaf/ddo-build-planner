use im::{OrdMap, OrdSet};
use utils::ord::IntoOrdGroupMap;

use crate::{
    attribute::{Attribute, TrackAttribute},
    bonus::{Bonus, BonusSource, CloneBonus},
};

#[derive(Default)]
pub struct Buffer {
    bonuses: OrdMap<BonusSource, Vec<Bonus>>,
    attributes: OrdSet<Attribute>,
    forced: OrdSet<Attribute>,
}

// PERF: Use the "children" design like the actual compiler does

impl Buffer {
    /// Inserts attributes into the queue. All attributes are forced as no bonuses are included
    pub fn insert_attributes<T>(&mut self, attributes: T)
    where
        T: Iterator<Item = Attribute>,
    {
        for attribute in attributes {
            self.attributes.insert(attribute);
            self.forced.insert(attribute);
        }
    }

    pub fn insert_bonuses<T>(&mut self, bonuses: T, forced: bool)
    where
        T: Iterator<Item = Bonus>,
    {
        let by_source = bonuses
            .flat_map(|bonus| {
                [
                    bonus
                        .get_attribute()
                        .clone_bonus(&bonus)
                        .unwrap_or_default(),
                    vec![bonus],
                ]
            })
            .flatten()
            .filter_map(|bonus| {
                bonus
                    .get_attribute()
                    .is_tracked()
                    .then_some((bonus.get_source(), bonus))
            })
            .into_grouped_ord_map();

        for (source, bonuses) in by_source {
            for bonus in &bonuses {
                let attribute = bonus.get_attribute();
                self.attributes.insert(attribute);
                if forced {
                    self.forced.insert(attribute);
                }
            }

            self.bonuses.insert(source, bonuses);
        }
    }

    pub fn pop(&mut self) -> Option<(Attribute, Vec<Bonus>, bool)> {
        loop {
            let attribute = self.attributes.remove_min()?;
            let forced = self.forced.remove(&attribute).is_some();
            let bonuses = self
                .bonuses
                .iter()
                .flat_map(|(_, bonuses)| {
                    bonuses
                        .iter()
                        .filter(|bonus| bonus.get_attribute().eq(&attribute))
                        .cloned()
                })
                .collect::<Vec<Bonus>>();

            if forced || !bonuses.is_empty() {
                return Some((attribute, bonuses, forced));
            }
        }
    }
}
