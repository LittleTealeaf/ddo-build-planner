use im::OrdSet;
use itertools::Itertools;

use crate::attribute::{Attribute, TrackAttribute};

#[derive(Default)]
pub struct AttributeQueue {
    forced: OrdSet<Attribute>,
    queue: OrdSet<Attribute>,
}

impl AttributeQueue {
    pub fn get_next_attribute(&mut self) -> Option<(Attribute, bool)> {
        let attribute = self.queue.remove_min()?;
        let forced = self.forced.remove(&attribute).is_some();
        Some((attribute, forced))
    }

    pub fn insert(&mut self, attributes: Vec<Attribute>, forced: bool) {
        let attributes = attributes
            .into_iter()
            .filter(Attribute::is_tracked)
            .collect_vec();

        if forced {
            self.forced.extend(attributes.iter());
        }

        self.queue.extend(attributes.into_iter());
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

        queue.insert(vec![Attribute::Debug(0)], false);

        let (attribute, _) = queue.get_next_attribute().expect("Expected Attribute");

        assert_eq!(attribute, Attribute::Debug(0));
    }

    #[test]
    fn duplicate_entries_do_not_stack() {
        let mut queue = AttributeQueue::default();

        queue.insert(vec![Attribute::Debug(0), Attribute::Debug(0)], false);

        assert_eq!(
            Some((Attribute::Debug(0), false)),
            queue.get_next_attribute()
        );
        assert_eq!(None, queue.get_next_attribute());
    }

    #[test]
    fn duplcate_entries_update_forced() {
        let mut queue = AttributeQueue::default();

        queue.insert(vec![Attribute::Debug(0)], false);
        queue.insert(vec![Attribute::Debug(0)], true);

        assert_eq!(
            Some((Attribute::Debug(0), true)),
            queue.get_next_attribute()
        );
        assert_eq!(None, queue.get_next_attribute());
    }

    #[test]
    fn forced_attributes_set_forced() {
        let mut queue = AttributeQueue::default();

        queue.insert(vec![Attribute::Debug(0)], true);

        assert_eq!(
            Some((Attribute::Debug(0), true)),
            queue.get_next_attribute()
        );
    }

    #[test]
    fn forced_attributes_lose_force_after_fetching() {
        let mut queue = AttributeQueue::default();

        queue.insert(vec![Attribute::Debug(0)], true);

        assert_eq!(
            Some((Attribute::Debug(0), true)),
            queue.get_next_attribute()
        );

        queue.insert(vec![Attribute::Debug(0)], false);

        assert_eq!(
            Some((Attribute::Debug(0), false)),
            queue.get_next_attribute()
        );
    }
}
