use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use super::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType, Condition},
};

/// Builds the child bonuses by taking the bonus, fetching the "clone" attributes from the bonus attribute, and creating a duplicate bonus for each one
macro_rules! build_child_bonuses {
    ($bonuses: expr) => {
        $bonuses.append(
            &mut $bonuses
                .iter()
                .filter_map(|bonus| {
                    Some(
                        bonus
                            .get_attribute()
                            .get_clone_attributes()?
                            .into_iter()
                            .map(|attribute| {
                                Bonus::new(
                                    attribute,
                                    bonus.get_bonus_type(),
                                    bonus.get_value(),
                                    bonus.get_source(),
                                    Some(bonus.get_conditions()),
                                )
                            }),
                    )
                })
                .flatten()
                .collect_vec(),
        );
    };
}

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
        if let Some(value) = self.cache.get(attribute) {
            return *value;
        }

        let value = self.calculate_attribute(attribute);

        self.cache.insert(*attribute, value);

        value
    }

    pub fn get_all_attributes(&mut self) -> HashMap<Attribute, f32> {
        self.bonuses
            .iter()
            .map(Bonus::get_attribute)
            .unique()
            .collect_vec()
            .into_iter()
            .map(|attribute| (attribute, self.get_attribute(&attribute)))
            .collect()
    }

    fn calculate_attribute(&self, attribute: &Attribute) -> f32 {
        let mut values = HashMap::new();

        self.bonuses
            .iter()
            .filter(|bonus| {
                bonus.get_attribute().eq(attribute)
                    && (bonus.get_conditions().iter().all(
                        |condition: &Condition| match condition {
                            Condition::Has(attr) => self.calculate_attribute(attr) > 0f32,
                            Condition::NotHave(attr) => self.calculate_attribute(attr) == 0f32,
                            Condition::Eq(attr, value) => self.calculate_attribute(attr) == *value,
                            Condition::Max(attr, value) => self.calculate_attribute(attr) >= *value,
                            Condition::Min(attr, value) => self.calculate_attribute(attr) <= *value,
                            Condition::NotEq(attr, value) => {
                                self.calculate_attribute(attr) != *value
                            }
                        },
                    ))
            })
            .for_each(|bonus| {
                if bonus.get_bonus_type().eq(&BonusType::Stacking) {
                    let previous_value = values.remove(&BonusType::Stacking).unwrap_or(0f32);
                    values.insert(BonusType::Stacking, previous_value + bonus.get_value());
                } else {
                    let value = bonus.get_value();
                    if &value > values.get(&bonus.get_bonus_type()).unwrap_or(&0f32) {
                        values.insert(bonus.get_bonus_type(), value);
                    }
                }
            });

        values.into_values().sum()
    }

    pub fn insert_bonuses(&mut self, mut bonuses: Vec<Bonus>) {
        // Appends "Cloned Bonuses" created from the "get_clone_attributes" function for each of
        // the bonuses
        build_child_bonuses!(bonuses);

        // The queue of attributes that still need to be processed
        let mut attribute_queue = bonuses
            .iter()
            .map(Bonus::get_attribute)
            .unique()
            .map(|item| (item, true))
            .collect::<VecDeque<_>>();

        // Remove all previous bonuses in the breakdown with the same sources
        {
            let sources = bonuses.iter().map(Bonus::get_source).unique().collect_vec();

            self.bonuses
                .iter()
                .enumerate()
                .filter(|(_, item)| sources.contains(&item.get_source()))
                .map(|(i, _)| i)
                .enumerate()
                .collect_vec()
                .into_iter()
                .map(|(n, i)| self.bonuses.swap_remove(i - n).get_attribute())
                .unique()
                .for_each(|attribute| {
                    if !attribute_queue.iter().any(|(item, _)| item.eq(&attribute)) {
                        attribute_queue.push_back((attribute, false));
                    }
                });
        }

        // The set of bonuses that still need to be inserted into the breakdown
        let mut update_bonuses = bonuses
            .into_iter()
            .map(|bonus| (bonus.get_attribute(), bonus))
            .into_group_map();

        // Fetch the next attribute to update
        while let Some((attribute, force_update)) = attribute_queue.pop_front() {
            // Fetches the initial value. If we're forcing the update, we won't care about it if
            // it's not stored in the cache
            let initial_value = {
                if let Some(value) = self.cache.remove(&attribute) {
                    value
                } else if force_update {
                    0f32
                } else {
                    self.calculate_attribute(&attribute)
                }
            };
            // Inserts the updated bonuses into the stack if they're not 0
            if let Some(bonuses) = update_bonuses.remove(&attribute) {
                for bonus in bonuses {
                    if bonus.get_value() != 0f32 {
                        self.bonuses.push(bonus);
                    }
                }
            }

            // If it's forced updte, or if the initial value is not equal to the current value.
            // This will coincidentially load the attribute (if we're not forcing updates)
            if force_update || initial_value != self.get_attribute(&attribute) {
                self.bonuses
                    .iter()
                    .filter(|bonus| {
                        bonus
                            .get_conditions()
                            .iter()
                            .any(|condition| match condition {
                                Condition::Has(attr)
                                | Condition::NotHave(attr)
                                | Condition::NotEq(attr, _)
                                | Condition::Eq(attr, _)
                                | Condition::Max(attr, _)
                                | Condition::Min(attr, _) => attribute.eq(attr),
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

                //Builds the source for any children bonuses
                let source = BonusSource::Attribute(attribute);

                // Removes any bonuses that have a source as this attribute
                self.bonuses
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| item.get_source().eq(&source))
                    .map(|(i, _)| i)
                    .enumerate()
                    .collect_vec()
                    .into_iter()
                    .map(|(n, i)| self.bonuses.swap_remove(i - n).get_attribute())
                    .unique()
                    .for_each(|attribute| {
                        if !attribute_queue.iter().any(|(item, _)| item.eq(&attribute)) {
                            attribute_queue.push_back((attribute, false));
                        }
                    });

                // Checks if there are any children bonuses
                if let Some(mut bonuses) =
                    attribute.get_attribute_bonuses(self.get_attribute(&attribute))
                {
                    // Includes any cloned attributes into the bonuses list
                    build_child_bonuses!(bonuses);

                    // Groups bonuses by attribute, and inserts them into the HashMap
                    // accordingly
                    bonuses
                        .into_iter()
                        .map(|bonus| (bonus.get_attribute(), bonus))
                        .into_group_map()
                        .into_iter()
                        .for_each(|(attribute, mut insert_bonuses)| {
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
                        });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::attribute::{Ability, WeaponHand, WeaponStat};

    use super::*;

    #[test]
    fn attribute_fetches() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::AbilityScore(Ability::Wisdom),
            BonusType::Feat,
            20f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::AbilityScore(Ability::Wisdom)),
            20f32
        );
    }

    #[test]
    fn cloned_attributes() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::WeaponStat(WeaponHand::Both, WeaponStat::Attack),
            BonusType::Stacking,
            10f32,
            BonusSource::Unique(2),
            None,
        )]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::WeaponStat(
                WeaponHand::MainHand,
                WeaponStat::Attack
            )),
            10f32
        );
    }

    #[test]
    fn same_types_dont_stack() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonuses(vec![
            Bonus::new(
                Attribute::AbilityScore(Ability::Charisma),
                BonusType::Enhancement,
                10f32,
                BonusSource::Unique(0),
                None,
            ),
            Bonus::new(
                Attribute::AbilityScore(Ability::Charisma),
                BonusType::Enhancement,
                5f32,
                BonusSource::Unique(1),
                None,
            ),
        ]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::AbilityScore(Ability::Charisma)),
            10f32
        );
    }

    #[test]
    fn different_types_stack() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonuses(vec![
            Bonus::new(
                Attribute::AbilityScore(Ability::Charisma),
                BonusType::Enhancement,
                10f32,
                BonusSource::Unique(0),
                None,
            ),
            Bonus::new(
                Attribute::AbilityScore(Ability::Charisma),
                BonusType::Insightful,
                5f32,
                BonusSource::Unique(1),
                None,
            ),
        ]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::AbilityScore(Ability::Charisma)),
            15f32
        );
    }

    #[test]
    fn stacking_types_stack() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonuses(vec![
            Bonus::new(
                Attribute::AbilityScore(Ability::Strength),
                BonusType::Stacking,
                10f32,
                BonusSource::Unique(0),
                None,
            ),
            Bonus::new(
                Attribute::AbilityScore(Ability::Strength),
                BonusType::Stacking,
                5f32,
                BonusSource::Unique(1),
                None,
            ),
        ]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::AbilityScore(Ability::Strength)),
            15f32
        );
    }

    #[test]
    fn replacing_source_removes_all_attributes() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonuses(vec![
            Bonus::new(
                Attribute::AbilityScore(Ability::Wisdom),
                BonusType::Stacking,
                10f32,
                BonusSource::Unique(0),
                None,
            ),
            Bonus::new(
                Attribute::AbilityScore(Ability::Charisma),
                BonusType::Stacking,
                10f32,
                BonusSource::Unique(0),
                None,
            ),
        ]);

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::AbilityScore(Ability::Wisdom),
            BonusType::Stacking,
            10f32,
            BonusSource::Unique(0),
            None,
        )]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::AbilityScore(Ability::Charisma)),
            0f32
        );
    }
}
