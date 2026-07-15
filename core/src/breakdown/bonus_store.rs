use std::collections::HashMap;

use im::OrdMap;
use itertools::chain;

use crate::{attribute::Attribute, bonus::Bonus};

#[derive(Debug, Clone)]
pub struct BonusStore(HashMap<Attribute, BonusEntry>);

#[derive(Debug, Clone)]
struct BonusEntry {
    core: Vec<Bonus>,
    configs: OrdMap<u32, Vec<Bonus>>,
}

impl BonusStore {
    pub fn get_bonuses(
        &self,
        attribute: &Attribute,
        config: Option<u32>,
    ) -> impl Iterator<Item = &Bonus> {
        self.0
            .get(&attribute)
            .map(|entry| {
                chain!(
                    entry.core.iter(),
                    config
                        .map(|cfg| entry.configs.get(&cfg))
                        .flatten()
                        .into_iter()
                        .flatten()
                )
            })
            .into_iter()
            .flatten()
    }
}
