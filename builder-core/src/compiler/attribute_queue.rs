use std::collections::{HashSet, VecDeque};

use crate::attribute::Attribute;

/// Handles the update queue for attributes whenever an attribute is inserted into a [Breakdowns](crate::breakdown::Breakdowns) object.
///
/// Attributes are handled on a first-in-first-out basis. Attributes in the queue are unique (there can not be multiple entries of the same attribute in the queue), and trying to add a duplicate attribute will simply not add anything.
///
/// The "Forced" variable is also tracked when adding to the queue. "Forced" indicates that the attribute should be forced to update, even if no updates were detected. If an attribute is already in the list, and a new entry is inserted where that attribute is forced, then when the attribute leaves the queue, it will be forced.
///
/// AttributeQueue is a privately used by [Breakdowns](crate::breakdown::Breakdowns).
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

    /// Fetches the next attribute
    ///
    ///
    pub fn get_next_attribute(&mut self) -> Option<(Attribute, bool)> {
        while let Some(attribute) = self.buffer.pop() {
            if !self.queue.contains(&attribute) {
                self.queue.push_back(attribute);
            }
        }

        let attribute = self.queue.pop_front()?;
        let forced = self.forced.remove(&attribute);
        Some((attribute, forced))
    }

    pub fn insert_attriubtes(&mut self, mut attributes: Vec<Attribute>, forced: bool) {
        if forced {
            for attribute in attributes.iter() {
                self.forced.insert(*attribute);
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
        let mut queue = AttributeQueue::new();

        assert_eq!(None, queue.get_next_attribute());
    }

    #[test]
    fn inserted_attributes_are_returned() {
        let mut queue = AttributeQueue::new();

        queue.insert_attriubtes(vec![Attribute::Dummy()], false);

        let (attribute, _) = queue.get_next_attribute().expect("get_next_attribute returned None");

        assert_eq!(Attribute::Dummy(), attribute);
    }

    #[test]
    fn inserted_attributes_returned_in_order() {
        let mut queue = AttributeQueue::new();

        queue.insert_attriubtes(
            vec![Attribute::MagicalSheltering(), Attribute::Dummy()],
            false,
        );
        queue.get_next_attribute();
        queue.insert_attriubtes(vec![Attribute::PhysicalSheltering()], false);

        assert_eq!(
            Some((Attribute::MagicalSheltering(), false)),
            queue.get_next_attribute()
        );
        assert_eq!(
            Some((Attribute::PhysicalSheltering(), false)),
            queue.get_next_attribute()
        );
    }

    #[test]
    fn duplicate_entries_do_not_stack() {
        let mut queue = AttributeQueue::new();

        queue.insert_attriubtes(vec![Attribute::MagicalSheltering()], false);
        queue.insert_attriubtes(vec![Attribute::MagicalSheltering()], false);

        assert_eq!(
            Some((Attribute::MagicalSheltering(), false)),
            queue.get_next_attribute()
        );
        assert_eq!(None, queue.get_next_attribute());
    }

    #[test]
    fn duplicate_entries_update_forced() {
        let mut queue = AttributeQueue::new();

        queue.insert_attriubtes(vec![Attribute::MagicalSheltering()], false);
        queue.insert_attriubtes(vec![Attribute::MagicalSheltering()], true);

        assert_eq!(
            Some((Attribute::MagicalSheltering(), true)),
            queue.get_next_attribute()
        );
        assert_eq!(None, queue.get_next_attribute());
    }

    #[test]
    fn forced_attributes_return_as_forced() {
        let mut queue = AttributeQueue::new();

        queue.insert_attriubtes(vec![Attribute::MagicalSheltering()], true);

        assert_eq!(
            Some((Attribute::MagicalSheltering(), true)),
            queue.get_next_attribute()
        );
    }

    #[test]
    fn forcing_attribute_only_updates_attribute() {
        let mut queue = AttributeQueue::new();

        queue.insert_attriubtes(
            vec![Attribute::MagicalSheltering(), Attribute::Dummy()],
            false,
        );
        queue.get_next_attribute();
        queue.insert_attriubtes(vec![Attribute::PhysicalSheltering()], true);

        assert_eq!(
            Some((Attribute::MagicalSheltering(), false)),
            queue.get_next_attribute()
        );
        assert_eq!(
            Some((Attribute::PhysicalSheltering(), true)),
            queue.get_next_attribute()
        );
    }
}
