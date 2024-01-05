use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource},
};

#[derive(Default)]
pub struct Buffer {
    attributes: BinaryHeap<Reverse<Attribute>>,
    forced: HashSet<Attribute>,
    bonuses: Vec<Bonus>,
}

impl Buffer {
    pub fn create(bonuses: impl IntoIterator<Item = Bonus>) -> Self {
        let mut buffer = Self::default();

        buffer.bonuses = bonuses
            .into_iter()
            .map(|bonus| {
                buffer.forced.insert(*bonus.get_attribute());
                buffer.attributes.push(Reverse(*bonus.get_attribute()));
                bonus
            })
            .collect();

        buffer
    }

    pub fn insert_attributes(&mut self, attributes: impl IntoIterator<Item = Attribute>) {
        for attribute in attributes {
            self.attributes.push(Reverse(attribute));
            self.forced.insert(attribute);
        }
    }

    pub fn insert_bonuses(&mut self, bonuses: impl IntoIterator<Item = Bonus>) {
        let bonuses = Vec::from_iter(bonuses);

        let sources: HashSet<BonusSource> =
            bonuses.iter().map(Bonus::get_source).copied().collect();
        self.bonuses.retain(|i| !sources.contains(i.get_source()));

        {
            let attributes: HashSet<Attribute> =
                bonuses.iter().map(Bonus::get_attribute).copied().collect();

            self.attributes.extend(attributes.into_iter().map(Reverse));
        }

        self.bonuses.extend(bonuses);
    }

    pub fn get_bonuses(&self) -> impl Iterator<Item = &Bonus> {
        self.bonuses.iter()
    }

    pub fn pop(&mut self) -> Option<(Attribute, Vec<Bonus>, bool)> {
        while let Some(Reverse(attribute)) = self.attributes.pop() {
            let mut bonuses = Vec::new();

            let mut i = 0;

            while i < self.bonuses.len() {
                if self.bonuses[i].get_attribute().eq(&attribute) {
                    bonuses.push(self.bonuses.swap_remove(i));
                } else {
                    i += 1;
                }
            }

            let forced = self.forced.remove(&attribute);

            if forced || !bonuses.is_empty() {
                return Some((attribute, bonuses, forced));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::bonus::BonusType;

    use super::*;

    #[test]
    fn empty_buffer_returns_none() {
        let mut buffer = Buffer::create([]);
        assert!(buffer.pop().is_none());
    }

    #[test]
    fn inserting_attribute_always_pops() {
        let mut buffer = Buffer::create([]);
        buffer.insert_attributes([Attribute::Debug(0)]);

        let (attribute, bonuses, forced) = buffer.pop().expect("Expected return from buffer.pop()");

        assert_eq!(attribute, Attribute::Debug(0));
        assert!(bonuses.is_empty());
        assert!(forced);
    }

    #[test]
    fn inserting_attribute_multiple_times_pops_once() {
        let mut buffer = Buffer::create([]);
        buffer.insert_attributes([Attribute::Debug(0), Attribute::Debug(0)]);

        let (attribute, _, _) = buffer.pop().expect("Expected return from buffer.pop()");
        assert_eq!(attribute, Attribute::Debug(0));
        assert!(buffer.pop().is_none());
    }

    #[test]
    fn creating_pops_bonuses() {
        let mut buffer = Buffer::create([Bonus::new(
            Attribute::Debug(0),
            BonusType::Stacking,
            1,
            BonusSource::Debug(0),
            None,
        )]);
        let (_, bonuses, _) = buffer.pop().expect("Expected return from buffer.pop()");
        assert!(!bonuses.is_empty());
        assert_eq!(
            &bonuses[0],
            &Bonus::new(
                Attribute::Debug(0),
                BonusType::Stacking,
                1,
                BonusSource::Debug(0),
                None,
            )
        );
    }

    #[test]
    fn attributes_pop_by_ord() {
        let mut buffer = Buffer::create([]);

        buffer.insert_attributes([Attribute::Debug(1), Attribute::Debug(0)]);

        let (attribute, _, _) = buffer.pop().expect("Expected return from buffer.pop()");
        assert_eq!(attribute, Attribute::Debug(0));
        let (attribute, _, _) = buffer.pop().expect("Expected return from buffer.pop()");
        assert_eq!(attribute, Attribute::Debug(1));
    }
}
