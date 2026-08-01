use std::collections::HashMap;

use im::OrdMap;
use itertools::{chain, Itertools};

use crate::{
    attribute::Attribute,
    bonus::{traits::ContainsAttribute, Bonus},
    bonuses::core_bonuses,
    breakdown::Breakdown,
    types::bonus_provider::BonusProvider,
};

#[derive(Debug, Clone)]
pub(super) struct BonusStore {
    bonuses: HashMap<Attribute, BonusEntry>,
    providers: HashMap<BonusProvider, Vec<Attribute>>,
}

impl Default for BonusStore {
    fn default() -> Self {
        Self {
            bonuses: core_bonuses()
                .chunk_by(|bonus| bonus.attribute().clone())
                .into_iter()
                .map(|(attribute, bonuses)| {
                    (
                        attribute,
                        BonusEntry {
                            core: bonuses.collect(),
                            snapshots: OrdMap::default(),
                        },
                    )
                })
                .collect(),
            providers: HashMap::default(),
        }
    }
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
        
    }

    pub fn get_dependant_attributes<'a>(
        &'a self,
        attribute: &'a Attribute,
    ) -> impl Iterator<Item = &'a Attribute> {
        self.bonuses
            .values()
            .flat_map(|entry| entry.core.iter().chain(entry.snapshots.values().flatten()))
            .filter(|bonus| bonus.contains_attribute(attribute))
            .map(Bonus::attribute)
            .unique()
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
