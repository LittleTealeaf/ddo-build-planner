use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use crate::{attribute::Attribute, bonus::Bonus};

#[derive(Default)]
pub struct Buffer {
    attributes: BinaryHeap<Reverse<Attribute>>,
    forced: HashSet<Attribute>,
    bonuses: Vec<Bonus>,
}

impl Buffer {
    pub fn insert_attributes(&mut self, attributes: impl IntoIterator<Item = Attribute>) {
        for attribute in attributes {
            self.attributes.push(Reverse(attribute));
            self.forced.insert(attribute);
        }
    }
    // TODO: continue
}
