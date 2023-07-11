use im::OrdSet;
use utils::ord::IntoOrdGroupMap;

use crate::{
    attribute::{Attribute, TrackAttribute},
    bonus::{Bonus, BonusSource},
};

#[derive(Default)]
pub struct Buffer {
    bonuses: Vec<Bonus>,
    sources: OrdSet<BonusSource>,
    attributes: OrdSet<Attribute>,
    forced: OrdSet<Attribute>,
}

impl Buffer {
    pub fn insert<T>(&mut self, bonuses: T, forced: bool)
    where
        T: Iterator<Item = Bonus>,
    {
        // This is getting a lot more complicated, because we also need to include a way for
        // attributes to be checked without having any bonuses attached to them

        let groued_by_source = bonuses
            .filter_map(|bonus| {
                bonus
                    .get_attribute()
                    .is_tracked()
                    .then_some((bonus.get_source(), bonus))
            })
            .into_grouped_ord_map();

        for (source, bonuses) in groued_by_source {
            if self.sources.insert(source).is_some() {
                self.bonuses.retain(|bonus| bonus.get_source().ne(&source))
            }

            bonuses
                .iter()
                .map(|bonus| bonus.get_attribute())
                .for_each(|attribute| {
                    self.attributes.insert(attribute);
                    if forced {
                        self.forced.insert(attribute);
                    }
                });
        }
    }
}
