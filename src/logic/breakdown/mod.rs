use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use super::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType, Condition},
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
        if let Some(value) = self.cache.get(attribute) {
            return *value;
        }

        let value = self.calculate_attribute(attribute);

        self.cache.insert(*attribute, value);

        value
    }

    fn calculate_attribute(&self, attribute: &Attribute) -> f32 {
        let mut values = HashMap::new();

        self.bonuses
            .iter()
            .filter(|bonus| {
                &bonus.get_attribute() == attribute
                    && (bonus.get_conditions().iter().all(|flag| match flag {
                        Condition::Has(attribute) => self.calculate_attribute(attribute) >= 0f32,
                        Condition::Eq(attribute, value) => {
                            self.calculate_attribute(attribute) == *value
                        }
                        Condition::Max(attribute, value) => {
                            self.calculate_attribute(attribute) >= *value
                        }
                        Condition::Min(attribute, value) => {
                            self.calculate_attribute(attribute) <= *value
                        }
                    }))
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

        values.iter().map(|(_, value)| value).sum()
    }

    pub fn insert_bonuses(&mut self, mut bonuses: Vec<Bonus>) {
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
                .for_each(|(n, i)| {
                    self.bonuses.swap_remove(i - n);
                });
        }

        bonuses.append(
            &mut bonuses
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
                    if bonus.get_value() != 0f32 {
                        self.bonuses.push(bonus);
                    }
                }
            }
            if force_update || initial_value != self.get_attribute(&attribute) {
                self.bonuses
                    .iter()
                    .filter(|bonus| {
                        bonus
                            .get_conditions()
                            .iter()
                            .any(|condition| match condition {
                                Condition::Has(attr)
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

                let source = BonusSource::Attribute(attribute);

                self.bonuses
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| item.get_source().eq(&source))
                    .map(|(i, _)| i)
                    .enumerate()
                    .collect_vec()
                    .into_iter()
                    .for_each(|(n, i)| {
                        self.bonuses.swap_remove(i - n);
                    });

                if let Some(mut bonuses) =
                    attribute.get_attribute_bonuses(self.get_attribute(&attribute))
                {
                    bonuses.append(
                        &mut bonuses
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
    use crate::logic::attribute::{Ability, WeaponStat};

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
            Attribute::WeaponStat(WeaponStat::Attack),
            BonusType::Stacking,
            10f32,
            BonusSource::Unique(2),
            None,
        )]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::MainHandWeapon(WeaponStat::Attack)),
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
}
