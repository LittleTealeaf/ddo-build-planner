use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::build::attribute::Attribute;

use super::{
    attribute::Flag,
    bonus::{condition::Condition, source::Source, types::BonusType, Bonus},
};

pub struct Breakdowns {
    bonuses: Vec<Bonus>,
    cache: HashMap<Attribute, f32>,
}

impl Breakdowns {
    pub fn new() -> Breakdowns {
        Self {
            bonuses: Vec::new(),
            cache: HashMap::new(),
        }
    }

    pub fn get_attribute(&mut self, attribute: &Attribute) -> f32 {
        if let Some(total) = self.cache.get(attribute) {
            return *total;
        }

        let value = self.calculate_attribute(attribute);

        self.cache.insert(*attribute, value);

        value
    }

    fn get_flags(&self) -> Vec<Flag> {
        self.bonuses
            .iter()
            .filter_map(|item| match item.get_attribute() {
                Attribute::Flag(flag) => Some(flag),
                _ => None,
            })
            .collect()
    }

    fn calculate_attribute(&self, attribute: &Attribute) -> f32 {
        let mut values: HashMap<BonusType, f32> = HashMap::new();

        let flags = self.get_flags();

        let bonuses = self.bonuses.iter().filter(|bonus| {
            &bonus.get_attribute() == attribute
                && (bonus.get_condition().iter().all(|flag| match flag {
                    Condition::Flag(flag) => (&flags).contains(flag),
                    Condition::NoFlag(flag) => !(&flags).contains(flag),
                }))
        });

        for bonus in bonuses {
            if bonus.get_bonus_type() == BonusType::Stacking
                || values.get(&bonus.get_bonus_type()).unwrap_or(&0f32) < &bonus.get_value()
            {
                values.insert(bonus.get_bonus_type(), bonus.get_value());
            }
        }

        values.values().sum()
    }

    pub fn insert_attributes(&mut self, attributes: Vec<Bonus>) {
        let mut update_attributes = attributes
            .iter()
            .map(Bonus::get_attribute)
            .map(|item| (item, true))
            .collect::<VecDeque<_>>();

        let mut update_bonuses: HashMap<Attribute, Vec<Bonus>> = HashMap::new();

        for bonus in attributes {
            let attribute = bonus.get_attribute();
            let mut bonuses = update_bonuses.remove(&attribute).unwrap_or(Vec::new());
            bonuses.push(bonus);
            update_bonuses.insert(attribute, bonuses);
        }

        while let Some((attribute, force_update)) = update_attributes.pop_front() {
            if let Some(bonuses) = update_bonuses.remove(&attribute) {
                let initial_value = self
                    .cache
                    .remove(&attribute)
                    .unwrap_or(self.calculate_attribute(&attribute));
                for bonus in bonuses {
                    self.bonuses.push(bonus);
                }
                let final_value = self.calculate_attribute(&attribute);

                if force_update || initial_value != final_value {
                    if let Attribute::Flag(flag) = attribute {
                        let dependant_attributes = self
                            .bonuses
                            .iter()
                            .filter(|bonus| {
                                bonus
                                    .get_condition()
                                    .iter()
                                    .filter_map(|condition| match condition {
                                        Condition::NoFlag(flag) | Condition::Flag(flag) => {
                                            Some(flag)
                                        }
                                    })
                                    .contains(&flag)
                            })
                            .map(|bonus| bonus.get_attribute())
                            .unique()
                            .collect::<Vec<_>>();
                        for attr in dependant_attributes {
                            update_attributes.push_back((attr, true))
                        }
                    }

                    for (n, i) in self
                        .bonuses
                        .iter()
                        .enumerate()
                        .filter(|(_, item)| item.get_source().eq(&Source::Attribute(attribute)))
                        .map(|(i, _)| i)
                        .enumerate()
                        .collect_vec()
                    {
                        self.bonuses.swap_remove(i - n);
                    }

                    let updates = attribute.get_bonuses(final_value);
                    let attributes = updates.iter().map(|update| update.get_attribute());
                    for attribute in attributes {
                        if update_attributes
                            .iter()
                            .filter(|(attr, _)| attr.eq(&attribute))
                            .count()
                            == 0
                        {
                            update_attributes.push_back((attribute, false));
                        }
                    }
                    for bonus in updates {
                        let attribute = bonus.get_attribute();
                        let mut bonuses = update_bonuses.remove(&attribute).unwrap_or(Vec::new());
                        bonuses.push(bonus);
                        update_bonuses.insert(attribute, bonuses);
                    }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.bonuses.clear();
    }

    pub fn get_all_attributes(&mut self) -> HashMap<Attribute, f32> {
        HashMap::from_iter(
            self.bonuses
                .iter()
                .map(|item| item.get_attribute())
                .unique()
                .map(|attribute| (attribute, (&self).calculate_attribute(&attribute))),
        )
    }
}
