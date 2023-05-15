use std::collections::HashMap;

use itertools::Itertools;

use crate::breakdown::attribute_queue::AttributeQueue;

use super::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType, Condition},
};

mod attribute_queue;

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
                            .get_attribute_clones()?
                            .into_iter()
                            .map(|attribute| {
                                Bonus::new(
                                    attribute,
                                    bonus.get_bonus_type(),
                                    bonus.get_value(),
                                    bonus.get_source(),
                                    bonus.get_conditions(),
                                )
                            }),
                    )
                })
                .flatten()
                .collect_vec(),
        );
    };
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Breakdowns {
    bonuses: Vec<Bonus>,
    #[serde(skip)]
    cache: HashMap<Attribute, f32>,
}

impl Default for Breakdowns {
    fn default() -> Self {
        Self::new()
    }
}

impl Breakdowns {
    pub fn new() -> Breakdowns {
        Self {
            bonuses: Vec::new(),
            cache: HashMap::new(),
        }
    }

    pub fn get_breakdown(&self, attribute: &Attribute) -> Vec<&Bonus> {
        self.bonuses
            .iter()
            .filter(|bonus| bonus.get_attribute().eq(attribute))
            .collect_vec()
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
        self.bonuses
            .iter()
            .filter(|bonus| {
                bonus.get_attribute().eq(attribute)
                    && bonus.get_conditions().map_or(true, |conditions| {
                        conditions.iter().all(|condition| match condition {
                            Condition::Has(attr) => self.calculate_attribute(attr) != 0f32,
                            Condition::NotHave(attr) => self.calculate_attribute(attr) == 0f32,
                            Condition::Eq(attr, value) => self.calculate_attribute(attr) == *value,
                            Condition::Max(attr, value) => self.calculate_attribute(attr) <= *value,
                            Condition::Min(attr, value) => self.calculate_attribute(attr) >= *value,
                            Condition::NotEq(attr, value) => {
                                self.calculate_attribute(attr) != *value
                            }
                        })
                    })
            })
            .map(|bonus| (bonus.get_bonus_type(), bonus.get_value()))
            .into_group_map()
            .into_iter()
            .map(|(bonus_type, items)| {
                if bonus_type.is_stacking() {
                    let mut total = 0f32;
                    for item in items {
                        total += item;
                    }
                    total
                } else {
                    let mut max = 0f32;
                    for item in items {
                        if max < item {
                            max = item;
                        }
                    }
                    max
                }
            })
            .sum()
    }

    pub fn insert_bonuses(&mut self, mut bonuses: Vec<Bonus>) {
        // Appends "Cloned Bonuses" created from the "get_clone_attributes" function for each of
        // the bonuses
        build_child_bonuses!(bonuses);

        // The queue of attributes that still need to be processed
        let mut attribute_queue = AttributeQueue::new();

        attribute_queue.insert_attributes(
            bonuses.iter().map(Bonus::get_attribute).unique().collect(),
            false,
        );

        // Remove all previous bonuses in the breakdown with the same sources
        {
            let sources = bonuses.iter().map(Bonus::get_source).unique().collect_vec();

            attribute_queue.insert_attributes(
                self.bonuses
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| sources.contains(&item.get_source()))
                    .map(|(i, _)| i)
                    .rev()
                    .collect_vec()
                    .into_iter()
                    .map(|i| self.bonuses.swap_remove(i).get_attribute())
                    .unique()
                    .collect(),
                true,
            );
        }

        // The set of bonuses that still need to be inserted into the breakdown
        let mut update_bonuses = bonuses
            .into_iter()
            .map(|bonus| (bonus.get_attribute(), bonus))
            .into_group_map();

        // Fetch the next attribute to update
        while let Some((attribute, force_update)) = attribute_queue.get_next_attribute() {
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
                self.bonuses.append(
                    &mut bonuses
                        .into_iter()
                        .filter(|bonus| bonus.get_value() != 0f32)
                        .collect(),
                )
            }

            // If it's forced update, or if the initial value is not equal to the current value.
            // This will coincidentially load the attribute (if we're not forcing updates)
            if force_update || initial_value != self.get_attribute(&attribute) {
                // Push any bonus attributes that have referenced the attribute to the queue
                attribute_queue.insert_attributes(
                    self.bonuses
                        .iter()
                        .filter(|bonus| {
                            bonus.get_conditions().map_or(false, |conditions| {
                                conditions.iter().any(|condition| match condition {
                                    Condition::Has(attr)
                                    | Condition::NotHave(attr)
                                    | Condition::NotEq(attr, _)
                                    | Condition::Eq(attr, _)
                                    | Condition::Max(attr, _)
                                    | Condition::Min(attr, _) => attribute.eq(attr),
                                })
                            })
                        })
                        .map(Bonus::get_attribute)
                        .unique()
                        .collect(),
                    true,
                );

                //Builds the source for any children bonuses
                let source = BonusSource::Attribute(attribute);

                // Removes any bonuses that have a source as this attribute
                attribute_queue.insert_attributes(
                    self.bonuses
                        .iter()
                        .enumerate()
                        .filter(|(_, item)| source.eq(&item.get_source()))
                        .map(|(i, _)| i)
                        .rev()
                        .collect_vec()
                        .into_iter()
                        .map(|i| self.bonuses.swap_remove(i).get_attribute())
                        .unique()
                        .collect(),
                    true,
                );

                let value = self.get_attribute(&attribute);

                // Checks if there are any children bonuses
                if let Some(mut bonuses) = attribute.get_attribute_bonuses(value) {
                    // Includes any cloned attributes into the bonuses list
                    build_child_bonuses!(bonuses);

                    // Groups bonuses by attribute, and inserts them into the HashMap
                    // accordingly
                    attribute_queue.insert_attributes(
                        bonuses
                            .into_iter()
                            .map(|bonus| (bonus.get_attribute(), bonus))
                            .into_group_map()
                            .into_iter()
                            .map(|(attribute, mut insert_bonuses)| {
                                insert_bonuses.append(
                                    &mut update_bonuses
                                        .remove(&attribute)
                                        .unwrap_or(Vec::new())
                                        .into_iter()
                                        .filter(|item| item.get_source().ne(&source))
                                        .collect_vec(),
                                );
                                update_bonuses.insert(attribute, insert_bonuses);
                                attribute
                            })
                            .unique()
                            .collect(),
                        false,
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::attribute::{Ability, Flag, WeaponHand, WeaponStat};

    use super::*;

    #[test]
    fn attribute_fetches() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::Ability(Ability::Wisdom),
            BonusType::Feat,
            20f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::Ability(Ability::Wisdom)),
            20f32
        );
    }

    #[test]
    fn cloned_attributes() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::WeaponStat(WeaponHand::Both, WeaponStat::Attack()),
            BonusType::Stacking,
            10f32,
            BonusSource::Unique(2),
            None,
        )]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::WeaponStat(
                WeaponHand::Main,
                WeaponStat::Attack()
            )),
            10f32
        );
    }

    #[test]
    fn same_types_dont_stack() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonuses(vec![
            Bonus::new(
                Attribute::Ability(Ability::Charisma),
                BonusType::Enhancement,
                10f32,
                BonusSource::Unique(0),
                None,
            ),
            Bonus::new(
                Attribute::Ability(Ability::Charisma),
                BonusType::Enhancement,
                5f32,
                BonusSource::Unique(1),
                None,
            ),
        ]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::Ability(Ability::Charisma)),
            10f32
        );
    }

    #[test]
    fn different_types_stack() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonuses(vec![
            Bonus::new(
                Attribute::Ability(Ability::Charisma),
                BonusType::Enhancement,
                10f32,
                BonusSource::Unique(0),
                None,
            ),
            Bonus::new(
                Attribute::Ability(Ability::Charisma),
                BonusType::Insightful,
                5f32,
                BonusSource::Unique(1),
                None,
            ),
        ]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::Ability(Ability::Charisma)),
            15f32
        );
    }

    #[test]
    fn stacking_types_stack() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonuses(vec![
            Bonus::new(
                Attribute::Ability(Ability::Strength),
                BonusType::Stacking,
                10f32,
                BonusSource::Unique(0),
                None,
            ),
            Bonus::new(
                Attribute::Ability(Ability::Strength),
                BonusType::Stacking,
                5f32,
                BonusSource::Unique(1),
                None,
            ),
        ]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::Ability(Ability::Strength)),
            15f32
        );
    }

    #[test]
    fn replacing_source_removes_all_attributes() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonuses(vec![
            Bonus::new(
                Attribute::Ability(Ability::Wisdom),
                BonusType::Stacking,
                10f32,
                BonusSource::Unique(0),
                None,
            ),
            Bonus::new(
                Attribute::Ability(Ability::Charisma),
                BonusType::Stacking,
                10f32,
                BonusSource::Unique(0),
                None,
            ),
        ]);

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::Ability(Ability::Wisdom),
            BonusType::Stacking,
            10f32,
            BonusSource::Unique(0),
            None,
        )]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::Ability(Ability::Charisma)),
            0f32
        );
    }

    #[test]
    fn sub_types_fetch() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::Ability(Ability::Constitution),
            BonusType::Stacking,
            20f32,
            BonusSource::Unique(0),
            None,
        )]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::AbilityModifier(Ability::Constitution)),
            5f32
        );
    }

    #[test]
    fn conditions_work() {
        let mut breakdowns = Breakdowns::new();

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::ClassLore(crate::attribute::ClassLore::Religious),
            BonusType::Stacking,
            10f32,
            BonusSource::Unique(0),
            None,
        )]);

        assert_eq!(
            0f32,
            breakdowns.get_attribute(&Attribute::MagicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::Flag(Flag::ReligiousLoreToQualityMagicalSheltering()),
            BonusType::Stacking,
            1f32,
            BonusSource::Unique(1),
            None,
        )]);

        assert_eq!(
            10f32,
            breakdowns.get_attribute(&Attribute::MagicalSheltering())
        );

        breakdowns.insert_bonuses(vec![Bonus::dummy(BonusSource::Unique(1))]);

        assert_eq!(
            0f32,
            breakdowns.get_attribute(&Attribute::MagicalSheltering())
        );
    }

    mod conditions {
        use super::*;

        #[test]
        fn has() {
            let mut breakdowns = Breakdowns::new();

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::PhysicalSheltering(),
                BonusType::Stacking,
                10f32,
                BonusSource::Unique(0),
                Some(vec![Condition::Has(Attribute::MagicalSheltering())]),
            )]);

            assert_eq!(
                0f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                1f32,
                BonusSource::Unique(1),
                None,
            )]);

            assert_eq!(
                10f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );
        }

        #[test]
        fn not_have() {
            let mut breakdowns = Breakdowns::new();

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::PhysicalSheltering(),
                BonusType::Stacking,
                10f32,
                BonusSource::Unique(0),
                Some(vec![Condition::NotHave(Attribute::MagicalSheltering())]),
            )]);

            assert_eq!(
                10f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                1f32,
                BonusSource::Unique(1),
                None,
            )]);

            assert_eq!(
                0f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );
        }

        #[test]
        fn max() {
            let mut breakdowns = Breakdowns::new();

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::PhysicalSheltering(),
                BonusType::Stacking,
                10f32,
                BonusSource::Unique(0),
                Some(vec![Condition::Max(Attribute::MagicalSheltering(), 2f32)]),
            )]);

            assert_eq!(
                10f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                1f32,
                BonusSource::Unique(1),
                None,
            )]);

            assert_eq!(
                10f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                2f32,
                BonusSource::Unique(1),
                None,
            )]);

            assert_eq!(
                10f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                3f32,
                BonusSource::Unique(1),
                None,
            )]);

            assert_eq!(
                0f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );
        }

        #[test]
        fn min() {
            let mut breakdowns = Breakdowns::new();

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::PhysicalSheltering(),
                BonusType::Stacking,
                10f32,
                BonusSource::Unique(0),
                Some(vec![Condition::Min(Attribute::MagicalSheltering(), 2f32)]),
            )]);

            assert_eq!(
                0f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                1f32,
                BonusSource::Unique(1),
                None,
            )]);

            assert_eq!(
                0f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                2f32,
                BonusSource::Unique(1),
                None,
            )]);

            assert_eq!(
                10f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                3f32,
                BonusSource::Unique(1),
                None,
            )]);

            assert_eq!(
                10f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );
        }

        #[test]
        fn eq() {
            let mut breakdowns = Breakdowns::new();

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::PhysicalSheltering(),
                BonusType::Stacking,
                10f32,
                BonusSource::Unique(0),
                Some(vec![Condition::Eq(Attribute::MagicalSheltering(), 2f32)]),
            )]);

            assert_eq!(
                0f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                1f32,
                BonusSource::Unique(1),
                None,
            )]);

            assert_eq!(
                0f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                2f32,
                BonusSource::Unique(1),
                None,
            )]);

            assert_eq!(
                10f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                3f32,
                BonusSource::Unique(1),
                None,
            )]);

            assert_eq!(
                0f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );
        }

        #[test]
        fn not_eq() {
            let mut breakdowns = Breakdowns::new();

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::PhysicalSheltering(),
                BonusType::Stacking,
                10f32,
                BonusSource::Unique(0),
                Some(vec![Condition::NotEq(Attribute::MagicalSheltering(), 2f32)]),
            )]);

            assert_eq!(
                10f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                1f32,
                BonusSource::Unique(1),
                None,
            )]);

            assert_eq!(
                10f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                2f32,
                BonusSource::Unique(1),
                None,
            )]);

            assert_eq!(
                0f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );

            breakdowns.insert_bonuses(vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                3f32,
                BonusSource::Unique(1),
                None,
            )]);

            assert_eq!(
                10f32,
                breakdowns.get_attribute(&Attribute::PhysicalSheltering())
            );
        }
    }
}
