use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::build::attribute::Attribute;

use self::updaters::get_updates;

use super::bonus::{Bonus, types::BonusType, source::Source};

mod updaters;

pub struct Breakdowns {
    bonuses: Vec<Bonus>,
}

impl Breakdowns {
    pub fn new() -> Breakdowns {
        Self {
            bonuses: Vec::new(),
        }
    }

    pub fn get_attribute(&self, attribute: &Attribute) -> f32 {
        let mut values: HashMap<BonusType, f32> = HashMap::new();

        let bonuses = self
            .bonuses
            .iter()
            .filter(|bonus| &bonus.get_attribute() == attribute);

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
            .collect::<VecDeque<_>>();

        let mut update_bonuses: HashMap<Attribute, Vec<Bonus>> = HashMap::new();

        for bonus in attributes {
            let attribute = bonus.get_attribute();
            let mut bonuses = update_bonuses.remove(&attribute).unwrap_or(Vec::new());
            bonuses.push(bonus);
            update_bonuses.insert(attribute, bonuses);
        }

        while let Some(attribute) = update_attributes.pop_front() {
            if let Some(bonuses) = update_bonuses.remove(&attribute) {
                let initial_value = self.get_attribute(&attribute);
                for bonus in bonuses {
                    self.bonuses.push(bonus);
                }
                let final_value = self.get_attribute(&attribute);

                if initial_value != final_value {
                    let remove_indices = self
                        .bonuses
                        .iter()
                        .enumerate()
                        .filter(|(_, item)| item.get_source().eq(&Source::Attribute(attribute)))
                        .map(|(i, _)| i)
                        .collect();
                    batch_remove(&mut self.bonuses, remove_indices);
                    let updates = get_updates(attribute, final_value);
                    let attributes = updates.iter().map(|update| update.get_attribute());
                    for attribute in attributes {
                        if !update_attributes.contains(&attribute) {
                            update_attributes.push_back(attribute);
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
}

fn batch_remove<T>(vec: &mut Vec<T>, indices: Vec<usize>) {
    for (n, i) in indices.into_iter().enumerate() {
        vec.swap_remove(i - n);
    }
}

#[cfg(test)]
mod tests {
    use crate::build::attribute::{ability::Ability, skill::Skill};

    use super::*;

    #[test]
    fn test_insert_attributes() {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_attributes(
            Bonus::new(
                Attribute::Ability(Ability::Strength),
                BonusType::Stacking,
                20.0,
                Source::Unique(0),
                None,
            )
            .into_vec(),
        );
        let value = breakdowns.get_attribute(&Attribute::AbilityModifier(Ability::Strength));
        assert_eq!(value, 5.0);

        breakdowns.insert_attributes(
            Bonus::new(
                Attribute::Ability(Ability::Wisdom),
                BonusType::Stacking,
                40.0,
                Source::Unique(0),
                None,
            )
            .into_vec(),
        );
        let value = breakdowns.get_attribute(&Attribute::Skill(Skill::Spot));
        println!("{}", value);
        assert_eq!(value, 15f32);
    }

    #[test]
    fn highest_value_is_used() {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_attributes(vec![
            Bonus::new(
                Attribute::Ability(Ability::Constitution),
                BonusType::Insightful,
                40.0,
                Source::Unique(1),
                None,
            ),
            Bonus::new(
                Attribute::Ability(Ability::Constitution),
                BonusType::Insightful,
                50.0,
                Source::Unique(1),
                None,
            ),
        ]);

        assert_eq!(
            breakdowns.get_attribute(&Attribute::Ability(Ability::Constitution)),
            50.0
        );
    }

    #[test]
    fn values_get_updated() {
        let mut breakdowns = Breakdowns::new();
        breakdowns.insert_attributes(
            Bonus::new(
                Attribute::Ability(Ability::Intelligence),
                BonusType::AbilityScore,
                20.0,
                Source::Unique(0),
                None,
            )
            .into_vec(),
        );
        assert_eq!(
            breakdowns.get_attribute(&Attribute::Ability(Ability::Intelligence)),
            20.0
        );
        breakdowns.insert_attributes(
            Bonus::new(
                Attribute::Ability(Ability::Intelligence),
                BonusType::Enhancement,
                13.0,
                Source::Unique(1),
                None,
            )
            .into_vec(),
        );
        assert_eq!(
            breakdowns.get_attribute(&Attribute::Ability(Ability::Intelligence)),
            33.0
        );
    }
}
