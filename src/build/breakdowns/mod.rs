use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::build::attribute::Attribute;

use super::{
    attribute::Toggle,
    bonus::{Bonus, BonusType, Condition, Source},
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

    fn calculate_attribute(&self, attribute: &Attribute) -> f32 {
        let mut values: HashMap<BonusType, f32> = HashMap::new();

        let bonuses = self.bonuses.iter().filter(|bonus| {
            &bonus.get_attribute() == attribute
                && (bonus.get_condition().iter().all(|flag| match flag {
                    Condition::Has(attribute) => self.calculate_attribute(attribute) > 0f32,
                    Condition::Equals(attribute, value) => {
                        self.calculate_attribute(attribute) == *value
                    }
                    Condition::Minimum(attribute, value) => {
                        self.calculate_attribute(attribute) >= *value
                    }
                    Condition::Maximum(attribute, value) => {
                        self.calculate_attribute(attribute) <= *value
                    }
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

    pub fn insert_bonuses(&mut self, bonuses: Vec<Bonus>) {
        {
            // Remove all previous bonuses in the breakdown that have the same sources
            let sources = bonuses
                .iter()
                .map(|item| item.get_source())
                .unique()
                .collect_vec();
            for (n, i) in self
                .bonuses
                .iter()
                .enumerate()
                .filter(|(_, item)| (&sources).contains(&item.get_source()))
                .map(|(i, _)| i)
                .enumerate()
                .collect_vec()
            {
                self.bonuses.swap_remove(i - n);
            }
        }

        // The queue of attributes that still need to be processed
        let mut attribute_queue = bonuses
            .iter()
            .map(Bonus::get_attribute)
            .unique()
            .map(|item| (item, true))
            .collect::<VecDeque<_>>();

        // The set of bonuses that still need to be inserted into the breakdown
        let mut update_bonuses = bonuses
            .into_iter()
            .map(|bonus| (bonus.get_attribute(), bonus))
            .into_group_map();

        while let Some((attribute, force_update)) = attribute_queue.pop_front() {
            let initial_value = {
                if let Some(value) = self.cache.remove(&attribute) {
                    value
                } else if force_update {
                    0f32
                } else {
                    self.calculate_attribute(&attribute)
                }
            };
            if let Some(bonuses) = update_bonuses.remove(&attribute) {
                for bonus in bonuses {
                    self.bonuses.push(bonus);
                }
            }

            let final_value = self.get_attribute(&attribute);

            if force_update || initial_value != final_value {
                self.bonuses
                    .iter()
                    .filter(|bonus| {
                        bonus
                            .get_condition()
                            .iter()
                            .any(|condition| match condition {
                                Condition::Has(attr)
                                | Condition::Equals(attr, _)
                                | Condition::Minimum(attr, _)
                                | Condition::Maximum(attr, _) => attribute.eq(attr),
                            })
                    })
                    .map(|bonus| bonus.get_attribute())
                    .unique()
                    .for_each(|attribute| {
                        let queue = &mut attribute_queue;
                        if !queue.iter().any(|(attr, _)| attr.eq(&attribute)) {
                            queue.push_back((attribute, true));
                        }
                    });

                let source = Source::Attribute(attribute);

                // Remove any attributes with this as a source
                for (n, i) in self
                    .bonuses
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| item.get_source().eq(&source))
                    .map(|(i, _)| i)
                    .enumerate()
                    .collect_vec()
                {
                    self.bonuses.swap_remove(i - n);
                }

                for (attribute, insert_bonuses) in attribute
                    .get_bonuses(self.get_attribute(&attribute))
                    .into_iter()
                    .map(|bonus| (bonus.get_attribute(), bonus))
                    .into_group_map()
                {
                    let mut insert_bonuses = insert_bonuses;
                    insert_bonuses.append(
                        &mut update_bonuses
                            .remove(&attribute)
                            .unwrap_or(Vec::new())
                            .into_iter()
                            .filter(|item| item.get_source().ne(&source))
                            .collect_vec(),
                    );
                    update_bonuses.insert(attribute, insert_bonuses);
                    if !attribute_queue.iter().any(|(item, _)| item.eq(&attribute)) {
                        attribute_queue.push_back((attribute, false));
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

    pub fn get_toggles(&mut self) -> Vec<Toggle> {
        self.bonuses
            .iter()
            .filter_map(|item| match item.get_attribute() {
                Attribute::Toggle(toggle) => Some(toggle),
                _ => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::build::attribute::Ability;

    use super::*;

    #[test]
    fn inserting_bonus_updates_attribute() {
        let mut breakdowns = Breakdowns::new();
        let initial = breakdowns.get_attribute(&Attribute::Attack());
        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::Attack(),
            BonusType::Stacking,
            10.0,
            Source::Unique(0),
            None,
        )]);
        let fin = breakdowns.get_attribute(&Attribute::Attack());

        assert_ne!(initial, fin);

        assert_eq!(breakdowns.get_attribute(&Attribute::Attack()), 10.0);
    }

    #[test]
    fn overwriting_source_removes_old_version() {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::Ability(Ability::Dexterity),
            BonusType::Stacking,
            10.0,
            Source::Unique(0),
            None,
        )]);
        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::Ability(Ability::Dexterity),
            BonusType::Stacking,
            5.0,
            Source::Unique(0),
            None,
        )]);
        let value = breakdowns.get_attribute(&Attribute::Ability(Ability::Dexterity));

        assert_eq!(value, 5.0);
    }

    #[test]
    fn different_types_stack() {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonuses(vec![
            Bonus::new(
                Attribute::Ability(Ability::Wisdom),
                BonusType::Enhancement,
                10.0,
                Source::Unique(0),
                None,
            ),
            Bonus::new(
                Attribute::Ability(Ability::Wisdom),
                BonusType::Quality,
                5.0,
                Source::Unique(0),
                None,
            ),
        ]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::Ability(Ability::Wisdom)),
            15.0
        );
    }
    #[test]
    fn same_type_does_not_stack() {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_bonuses(vec![
            Bonus::new(
                Attribute::Ability(Ability::Wisdom),
                BonusType::Enhancement,
                10.0,
                Source::Unique(0),
                None,
            ),
            Bonus::new(
                Attribute::Ability(Ability::Wisdom),
                BonusType::Enhancement,
                5.0,
                Source::Unique(0),
                None,
            ),
        ]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::Ability(Ability::Wisdom)),
            10.0
        );
    }
}
