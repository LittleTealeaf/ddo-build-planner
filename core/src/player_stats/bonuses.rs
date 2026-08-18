mod core;

use itertools::{chain, Itertools};
use std::collections::HashMap;

use crate::{
    attribute::Attribute,
    bonus::{traits::ContainsAttribute, Bonus},
    player_stats::{bonuses::core::core_bonuses, PlayerStats},
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
                .map(|bonus| {
                    (
                        bonus.attribute().clone(),
                        BonusEntry {
                            bonus,
                            provider: BonusProvider::Core,
                            snapshot: None,
                        },
                    )
                })
                .into_group_map(),
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

    pub fn clear_provider(&mut self, provider: &BonusProvider) -> Option<Vec<Attribute>> {
        let attributes = self.providers.remove(provider)?;
        for attribute in &attributes {
            if let Some(bonuses) = self.bonuses.get_mut(attribute) {
                bonuses.retain(|bonus| &bonus.provider != provider);
            }
        }
        Some(attributes)
    }

    pub fn insert_bonuses<I>(
        &mut self,
        bonuses: I,
        provider: BonusProvider,
        snapshot: Option<u32>,
    ) -> impl Iterator<Item = Attribute>
    where
        I: IntoIterator<Item = Bonus>,
    {
        let removed_attributes = self.clear_provider(&provider).unwrap_or_default();
        let mut new_attributes = Vec::new();

        for bonus in bonuses {
            new_attributes.push(bonus.attribute().clone());
            let att_entry = self.bonuses.entry(bonus.attribute().clone()).or_default();
            att_entry.push(BonusEntry {
                bonus,
                snapshot,
                provider: provider.clone(),
            });
        }

        self.providers.insert(provider, new_attributes.clone());

        chain!(removed_attributes, new_attributes)
    }

    pub fn get_dependant_attributes<'a>(
        &'a self,
        attribute: &'a Attribute,
    ) -> impl Iterator<Item = &'a Attribute> {
        self.bonuses.iter().filter_map(move |(attr, bonuses)| {
            bonuses
                .iter()
                .any(|entry| entry.bonus.contains_attribute(attribute))
                .then_some(attr)
        })
    }
}

impl PlayerStats {
    pub fn bonuses(&self) -> impl Iterator<Item = &Bonus> {
        self.bonuses
            .bonuses
            .values()
            .flatten()
            .map(|entry| &entry.bonus)
    }

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
