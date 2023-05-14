use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::attribute::Attribute;

pub struct AttributeQueue {
    forced: HashSet<Attribute>,
    queue: VecDeque<Attribute>,
}

impl AttributeQueue {
    pub fn new() -> AttributeQueue {
        Self {
            forced: HashSet::new(),
            queue: VecDeque::new(),
        }
    }

    pub fn get_next_attribute(&mut self) -> Option<(Attribute, bool)> {
        let attribute = self.queue.pop_front()?;
        let forced = self.forced.remove(&attribute);
        Some((attribute, forced))
    }

    pub fn insert_updates<T: Iterator<Item = Attribute>>(&mut self, attributes: T, forced: bool) {
        attributes.unique().for_each(|attribute| {
            if !self.queue.contains(&attribute) {
                self.queue.push_back(attribute);
            }
            if forced {
                self.forced.insert(attribute);
            }
        })
    }
}
