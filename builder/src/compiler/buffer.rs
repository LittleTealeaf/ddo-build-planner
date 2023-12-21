use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use itertools::Itertools;

use crate::{
    attribute::{Attribute, TrackAttribute},
    bonus::{Bonus, BonusSource, CloneBonus},
};

#[derive(Default)]
pub struct Buffer {
    attributes: BinaryHeap<Reverse<Attribute>>,
    forced: HashSet<Attribute>,
    bonuses: Vec<Bonus>,
}

impl Buffer {
    /// Inserts attributes into the queue. All attributes are forced as no bonuses are included
    pub fn insert_attributes(&mut self, attributes: impl IntoIterator<Item = Attribute>) {
        for attribute in attributes {
            self.attributes.push(Reverse(attribute));
            self.forced.insert(attribute);
        }
    }

    pub fn insert_bonuses(&mut self, bonuses: impl IntoIterator<Item = Bonus>, forced: bool) {
        let bonuses = bonuses
            .into_iter()
            .flat_map(|bonus| {
                [
                    bonus
                        .get_attribute()
                        .clone_bonus(&bonus)
                        .unwrap_or_default(),
                    vec![bonus],
                ]
            })
            .flatten()
            .filter(|bonus| bonus.get_attribute().is_tracked())
            .collect_vec();

        let sources: HashSet<BonusSource> = bonuses.iter().map(Bonus::get_source).collect();

        // Remove any residing bonuses from any of the provided sources
        {
            let indexes: Vec<usize> = self
                .bonuses
                .iter()
                .enumerate()
                .filter_map(|(index, bonus)| sources.contains(&bonus.get_source()).then_some(index))
                .rev()
                .collect();
            for index in indexes {
                self.bonuses.swap_remove(index);
            }
        }

        // Handles adding attributes to respective sets
        {
            let attributes: HashSet<Attribute> = bonuses.iter().map(Bonus::get_attribute).collect();

            if forced {
                self.forced.extend(&mut attributes.iter().copied());
            }

            self.attributes
                .extend(&mut attributes.into_iter().map(Reverse));
        }

        // Add all bonuses to the bonuses list
        self.bonuses.extend(bonuses);
    }

    pub fn pop(&mut self) -> Option<(Attribute, Vec<Bonus>, bool)> {
        while let Some(Reverse(attribute)) = self.attributes.pop() {
            let bonuses = {
                let indexes = self
                    .bonuses
                    .iter()
                    .enumerate()
                    .filter_map(|(index, bonus)| {
                        bonus.get_attribute().eq(&attribute).then_some(index)
                    })
                    .rev()
                    .collect_vec();
                indexes
                    .into_iter()
                    .map(|index| self.bonuses.swap_remove(index))
                    .collect_vec()
            };

            let forced = self.forced.remove(&attribute);

            if forced || !bonuses.is_empty() {
                return Some((attribute, bonuses, forced));
            }
        }
        None
    }

    pub fn get_sources(&self) -> HashSet<BonusSource> {
        self.bonuses.iter().map(Bonus::get_source).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::bonus::{BonusSource, BonusType};

    use super::*;

    #[test]
    fn inserting_attribute_is_forced() {
        let mut buffer = Buffer::default();

        buffer.insert_attributes([Attribute::Debug(0)]);
        let value = buffer.pop();
        assert!(value.is_some());
        let (attribute, _, forced) = value.unwrap();
        assert_eq!(attribute, Attribute::Debug(0));
        assert!(forced);
    }

    #[test]
    fn empty_buffer_pops_none() {
        let mut buffer = Buffer::default();
        assert!(buffer.pop().is_none());
        buffer.insert_attributes([Attribute::Debug(1)]);
        assert!(buffer.pop().is_some());
        assert!(buffer.pop().is_none());
    }

    #[test]
    fn inserting_bonus_pops_bonus() {
        let mut buffer = Buffer::default();
        buffer.insert_bonuses(
            [Bonus::new(
                Attribute::Debug(3),
                BonusType::Stacking,
                10f32.into(),
                BonusSource::Debug(0),
                None,
            )],
            false,
        );

        let value = buffer.pop();
        assert!(value.is_some());
        let (attribute, bonuses, forced) = value.unwrap();
        assert_eq!(attribute, Attribute::Debug(3));
        assert_eq!(bonuses.len(), 1);
        assert!(!forced);

        buffer.insert_bonuses(
            [
                Bonus::new(
                    Attribute::Debug(3),
                    BonusType::Stacking,
                    10f32.into(),
                    BonusSource::Debug(0),
                    None,
                ),
                Bonus::new(
                    Attribute::Debug(3),
                    BonusType::Stacking,
                    10f32.into(),
                    BonusSource::Debug(1),
                    None,
                ),
            ],
            true,
        );

        let value = buffer.pop();
        assert!(value.is_some());
        let (attribute, bonuses, forced) = value.unwrap();
        assert_eq!(attribute, Attribute::Debug(3));
        assert_eq!(bonuses.len(), 2);
        assert!(forced);
    }

    #[test]
    fn inserting_forced_overwrites() {
        let mut buffer: Buffer = Buffer::default();
        buffer.insert_bonuses(
            [Bonus::new(
                Attribute::Debug(3),
                BonusType::Stacking,
                10f32.into(),
                BonusSource::Debug(2),
                None,
            )],
            false,
        );

        buffer.insert_bonuses(
            [Bonus::new(
                Attribute::Debug(3),
                BonusType::Stacking,
                10f32.into(),
                BonusSource::Debug(0),
                None,
            )],
            true,
        );

        let value = buffer.pop();
        assert!(value.is_some());
        let (attribute, bonuses, forced) = value.unwrap();
        assert_eq!(attribute, Attribute::Debug(3));
        assert_eq!(bonuses.len(), 2);
        assert!(forced);
    }

    #[test]
    fn popping_removes_bonuses() {
        let mut buffer = Buffer::default();

        buffer.insert_bonuses(
            [Bonus::new(
                Attribute::Debug(3),
                BonusType::Stacking,
                10f32.into(),
                BonusSource::Debug(0),
                None,
            )],
            true,
        );

        let value = buffer.pop();
        assert!(value.is_some());
        let (_, bonuses, _) = value.unwrap();
        assert_eq!(bonuses.len(), 1);

        buffer.insert_attributes([Attribute::Debug(3)]);

        let value = buffer.pop();
        assert!(value.is_some());
        let (_, bonuses, _) = value.unwrap();
        assert!(bonuses.is_empty());
    }
}
