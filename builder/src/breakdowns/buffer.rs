use core::{cmp::Reverse, iter::once};
use std::collections::{BinaryHeap, HashSet};

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, CloneBonus},
};

#[derive(Default)]
pub struct Buffer {
    attributes: BinaryHeap<Reverse<Attribute>>,
    forced: HashSet<Attribute>,
    bonuses: Vec<Bonus>,
}

impl Buffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_attributes<A, I>(mut self, attributes: I) -> Self
    where
        A: Into<Attribute>,
        I: IntoIterator<Item = A>,
    {
        self.insert_attributes(attributes);
        self
    }

    pub fn with_bonuses<I>(mut self, bonuses: I) -> Self
    where
        I: IntoIterator<Item = Bonus>,
    {
        self.insert_bonuses(bonuses);
        self
    }

    pub fn insert_attributes<A, I>(&mut self, attributes: I)
    where
        A: Into<Attribute>,
        I: IntoIterator<Item = A>,
    {
        for attribute in attributes {
            let attribute: Attribute = attribute.into();
            self.attributes.push(Reverse(attribute.clone()));
            self.forced.insert(attribute);
        }
    }

    pub fn insert_bonuses<I>(&mut self, bonuses: I)
    where
        I: IntoIterator<Item = Bonus>,
    {
        let bonuses = bonuses
            .into_iter()
            .flat_map(|bonus| {
                bonus
                    .attribute()
                    .clone_bonus(&bonus)
                    .into_iter()
                    .flatten()
                    .chain(once(bonus))
            })
            .collect::<Vec<_>>();

        let sources: HashSet<BonusSource> = bonuses.iter().map(Bonus::source).cloned().collect();
        self.bonuses.retain(|i| !sources.contains(i.source()));

        {
            let attributes: HashSet<Attribute> =
                bonuses.iter().map(Bonus::attribute).cloned().collect();

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
                if self.bonuses[i].attribute().eq(&attribute) {
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
    use crate::debug::DebugValue;

    use super::*;

    #[test]
    fn empty_buffer_returns_none() {
        let mut buffer = Buffer::new();
        assert!(buffer.pop().is_none());
    }

    #[test]
    fn inserting_attribute_always_pops() {
        let mut buffer = Buffer::new();
        buffer.insert_attributes([Attribute::Debug(0)]);

        let (attribute, bonuses, forced) = buffer.pop().expect("Expected return from buffer.pop()");

        assert_eq!(attribute, Attribute::Debug(0));
        assert!(bonuses.is_empty());
        assert!(forced);
    }

    #[test]
    fn inserting_attribute_multiple_times_pops_once() {
        let mut buffer = Buffer::new();
        buffer.insert_attributes([Attribute::Debug(0), Attribute::Debug(0)]);

        let (attribute, _, _) = buffer.pop().expect("Expected return from buffer.pop()");
        assert_eq!(attribute, Attribute::Debug(0));
        assert!(buffer.pop().is_none());
    }

    #[test]
    fn attributes_pop_by_ord() {
        let mut buffer = Buffer::new();

        buffer.insert_attributes([Attribute::Debug(1), Attribute::Debug(0)]);

        let (attribute, _, _) = buffer.pop().expect("Expected return from buffer.pop()");
        assert_eq!(attribute, Attribute::Debug(0));
        let (attribute, _, _) = buffer.pop().expect("Expected return from buffer.pop()");
        assert_eq!(attribute, Attribute::Debug(1));
    }

    #[test]
    fn inserting_bonus_pops() {
        let mut buffer = Buffer::new();

        buffer.insert_bonuses([Bonus::new(
            DebugValue(0),
            DebugValue(0),
            1,
            DebugValue(0),
            None,
        )]);
    }
}
