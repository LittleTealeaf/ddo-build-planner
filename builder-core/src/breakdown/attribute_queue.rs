use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::attribute::Attribute;

pub struct AttributeQueue {
    forced: HashSet<Attribute>,
    queue: VecDeque<Attribute>,
    buffer: Vec<Attribute>,
}

impl AttributeQueue {
    pub fn new() -> AttributeQueue {
        Self {
            forced: HashSet::new(),
            queue: VecDeque::new(),
            buffer: Vec::new(),
        }
    }

    pub fn get_next_attribute(&mut self) -> Option<(Attribute, bool)> {
        self.buffer.iter().unique().for_each(|attribute| {
            if !self.queue.contains(attribute) {
                self.queue.push_back(*attribute)
            }
        });

        self.buffer.clear();

        let attribute = self.queue.pop_front()?;
        let forced = self.forced.remove(&attribute);
        Some((attribute, forced))
    }

    pub fn insert_updates<T: Iterator<Item = Attribute>>(&mut self, attributes: T, forced: bool) {
        let mut attributes: Vec<Attribute> = attributes.collect();

        if forced {
            for attribute in attributes.iter() {
                self.forced.insert(*attribute);
            }
        }

        self.buffer.append(&mut attributes);
    }
}
