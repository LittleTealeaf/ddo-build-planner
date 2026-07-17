use std::collections::HashMap;

use im::OrdMap;
use itertools::chain;

use crate::{
    attribute::Attribute, bonus::Bonus, breakdown::Breakdown, types::bonus_provider::BonusProvider,
};

#[derive(Debug, Clone)]
pub struct BonusStore {
    bonuses: HashMap<Attribute, BonusEntry>,
    providers: HashMap<BonusProvider, Vec<Attribute>>,
}

#[derive(Debug, Clone)]
struct BonusEntry {
    core: Vec<Bonus>,
    snapshots: OrdMap<u32, Vec<Bonus>>,
}

impl BonusStore {
    pub fn get_bonuses(
        &self,
        attribute: &Attribute,
        snapshot: Option<u32>,
    ) -> impl Iterator<Item = &Bonus> {
        self.bonuses
            .get(attribute)
            .map(|entry| {
                chain!(
                    entry.core.iter(),
                    snapshot
                        .and_then(|cfg| entry.snapshots.get(&cfg))
                        .into_iter()
                        .flatten()
                )
            })
            .into_iter()
            .flatten()
    }

    pub fn insert_bonuses<I>(&mut self, bonuses: I, provider: BonusProvider, snapshot: Option<u32>)
    where
        I: IntoIterator<Item = Bonus>,
    {
        for bonus in bonuses {}
    }
}

impl Breakdown {
    pub fn get_bonuses(&self, attribute: &Attribute) -> impl Iterator<Item = &Bonus> {
        self.bonuses.get_bonuses(attribute, None)
    }

    pub fn get_snapshot_bonuses(
        &self,
        attribute: &Attribute,
        snapshot: u32,
    ) -> impl Iterator<Item = &Bonus> {
        self.bonuses.get_bonuses(attribute, Some(snapshot))
    }
}
