use core::hash::Hash;
use std::collections::HashMap;

use super::MapGetOrDefault;

/// Provides the `.into_grouped_hash_map` function for iterators
pub trait IntoGroupedHashMap<K, V>
where
    K: Hash + Eq + PartialEq + Clone,
{
    /// Converts this into a grouped hash map
    fn into_grouped_hash_map(self) -> HashMap<K, Vec<V>>;
}

impl<K, V, I> IntoGroupedHashMap<K, V> for I
where
    K: Hash + Eq + PartialEq + Clone,
    I: IntoIterator<Item = (K, V)>,
{
    fn into_grouped_hash_map(self) -> HashMap<K, Vec<V>> {
        let mut map: HashMap<K, Vec<V>> = HashMap::new();
        for (key, value) in self {
            map.get_mut_or_default(&key).push(value);
        }
        map
    }
}
