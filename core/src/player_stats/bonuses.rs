use itertools::Itertools;
use std::collections::HashMap;

use crate::{
    attribute::Attribute,
    bonus::{traits::ContainsAttribute, Bonus},
    bonuses::core_bonuses,
    player_stats::PlayerStats,
    types::bonus_provider::BonusProvider,
};

#[derive(Debug, Clone)]
pub(super) struct Bonuses {
    bonuses: HashMap<Attribute, Vec<BonusEntry>>,
    providers: HashMap<BonusProvider, Vec<Attribute>>,
}

impl Default for Bonuses {
    fn default() -> Self {
        Self {
            bonuses: core_bonuses()
                .chunk_by(|bonus| bonus.attribute().clone())
                .into_iter()
                .map(|(attribute, bonuses)| {
                    (
                        attribute,
                        bonuses
                            .into_iter()
                            .map(|bonus| BonusEntry {
                                bonus,
                                provider: BonusProvider::Core,
                                snapshot: None,
                            })
                            .collect(),
                    )
                })
                .collect(),
            providers: HashMap::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct BonusEntry {
    pub bonus: Bonus,
    pub provider: BonusProvider,
    pub snapshot: Option<u32>,
}

impl Bonuses {
    pub fn get_snapshots(&self, attribute: &Attribute) -> im::OrdSet<u32> {
        self.get_bonuses(attribute)
            .into_iter()
            .flatten()
            .filter_map(|bonus| bonus.snapshot)
            .collect()
    }

    pub fn get_bonuses(&self, attribute: &Attribute) -> Option<&Vec<BonusEntry>> {
        self.bonuses.get(attribute)
    }

    pub fn get_snapshot_bonuses(
        &self,
        attribute: &Attribute,
        snapshot: Option<u32>,
    ) -> impl Iterator<Item = &Bonus> {
        self.bonuses
            .get(attribute)
            .into_iter()
            .flatten()
            .filter_map(move |entry| {
                entry
                    .snapshot
                    .is_none_or(|snap| Some(snap) == snapshot)
                    .then_some(&entry.bonus)
            })
    }

    pub fn insert_bonuses<I>(
        &mut self,
        bonuses: I,
        provider: &BonusProvider,
        snapshot: Option<u32>,
    ) -> impl Iterator<Item = Attribute> + '_
    where
        I: IntoIterator<Item = Bonus>,
    {
        let entry = self.providers.entry(provider.clone()).or_default();
        let removed_attributes = entry.clone();
        entry.clear();
        for bonus in bonuses {
            entry.push(bonus.attribute().clone());
            let att_entry = self.bonuses.entry(bonus.attribute().clone()).or_default();
            att_entry.push(BonusEntry {
                bonus,
                snapshot,
                provider: provider.clone(),
            });
        }

        entry.iter().cloned().chain(removed_attributes)
    }

    pub fn get_dependant_attributes<'a>(
        &'a self,
        attribute: &'a Attribute,
    ) -> impl Iterator<Item = &'a Attribute> {
        self.bonuses.iter().filter_map(move |(attr, bonuses)| {
            bonuses
                .iter()
                .any(|entry| entry.bonus.contains_attribute(attr))
                .then_some(attribute)
        })
    }
}

impl PlayerStats {
    pub fn get_bonuses(&self, attribute: &Attribute) -> impl Iterator<Item = &Bonus> {
        self.bonuses.get_snapshot_bonuses(attribute, None)
    }

    pub fn get_snapshot_bonuses(
        &self,
        attribute: &Attribute,
        snapshot: u32,
    ) -> impl Iterator<Item = &Bonus> {
        self.bonuses.get_snapshot_bonuses(attribute, Some(snapshot))
    }
}
