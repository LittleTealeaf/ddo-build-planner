use std::collections::VecDeque;

use crate::{attribute::Attribute, utils::EnumBinaryMap};

#[derive(Default)]
pub struct AttributeQueue {
    forced: EnumBinaryMap<Attribute, ()>,
    queue: VecDeque<Attribute>,
    buffer: Vec<Attribute>,
}

impl AttributeQueue {
    pub fn get_next_attribute(&mut self) -> Option<(Attribute, bool)> {
        while let Some(attribute) = self.buffer.pop() {
            if !self.queue.contains(&attribute) {
                self.queue.push_back(attribute);
            }
        }

        let attribute = self.queue.pop_front()?;
        let forced = self.forced.remove(&attribute).is_some();
        Some((attribute, forced))
    }

    pub fn insert(&mut self, mut attributes: Vec<Attribute>, forced: bool) {
        if forced {
            for attribute in &attributes {
                self.forced.insert(*attribute, ());
            }
        }

        self.buffer.append(&mut attributes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_queue_returns_none() {
        let mut queue = AttributeQueue::default();

        assert_eq!(None, queue.get_next_attribute());
    }

    #[test]
    fn inserted_attributes_are_returned() {
        let mut queue = AttributeQueue::default();

        queue.insert(vec![Attribute::Dummy], false);

        let (attribute, _) = queue.get_next_attribute().expect("Expected Attribute");

        assert_eq!(attribute, Attribute::Dummy);
    }

    #[test]
    fn duplicate_entries_do_not_stack() {
        let mut queue = AttributeQueue::default();

        queue.insert(vec![Attribute::Dummy, Attribute::Dummy], false);

        assert_eq!(Some((Attribute::Dummy, false)), queue.get_next_attribute());
        assert_eq!(None, queue.get_next_attribute());
    }

    #[test]
    fn duplcate_entries_update_forced() {
        let mut queue = AttributeQueue::default();

        queue.insert(vec![Attribute::Dummy], false);
        queue.insert(vec![Attribute::Dummy], true);

        assert_eq!(Some((Attribute::Dummy, true)), queue.get_next_attribute());
        assert_eq!(None, queue.get_next_attribute());
    }

    #[test]
    fn forced_attributes_set_forced() {
        let mut queue = AttributeQueue::default();

        queue.insert(vec![Attribute::Dummy], true);

        assert_eq!(Some((Attribute::Dummy, true)), queue.get_next_attribute());
    }

    #[test]
    fn forced_attributes_lose_force_after_fetching() {
        let mut queue = AttributeQueue::default();

        queue.insert(vec![Attribute::Dummy], true);

        assert_eq!(Some((Attribute::Dummy, true)), queue.get_next_attribute());

        queue.insert(vec![Attribute::Dummy], false);

        assert_eq!(Some((Attribute::Dummy, false)), queue.get_next_attribute());
    }
}
