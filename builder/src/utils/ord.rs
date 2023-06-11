//! Implements various traits to simplify conversion from and to [`OrdMap`] and [`OrdSet`]
//!
//! [`OrdMap`]: im::OrdMap
//! [`OrdSet`]: im::OrdSet

use im::{OrdMap, OrdSet};

/// Implements the ability to convert a type to an [`OrdMap`]
pub trait ToGroupOrdMap<K, V>
where
    K: Ord,
{
    /// Converts the current object into an [`OrdMap`]
    fn into_grouped_ord_map(self) -> OrdMap<K, V>;
}

impl<I, K, V> ToGroupOrdMap<K, Vec<V>> for I
where
    I: Iterator<Item = (K, V)>,
    K: Ord + Clone,
    V: Clone,
{
    /// Converts into an [`OrdMap`] grouped by the key.
    ///
    /// The returning map is an [`OrdMap`] that maps a key to a [`Vec`] of `V` items (grouped by
    /// `K`)
    ///
    fn into_grouped_ord_map(self) -> OrdMap<K, Vec<V>> {
        let mut map: OrdMap<K, Vec<V>> = OrdMap::new();

        self.for_each(|(key, value)| {
            if let Some(set) = map.get_mut(&key) {
                set.push(value);
            } else {
                map.insert(key, vec![value]);
            }
        });

        map
    }
}

/// Implements the ability to convert a type to an [`OrdSet`]
pub trait ToGroupedOrdSet<K>
where
    K: Ord,
{
    /// Converts into an [`OrdSet`]
    fn into_grouped_ord_set(self) -> OrdSet<K>;
}

impl<I, K> ToGroupedOrdSet<K> for I
where
    I: Iterator<Item = K>,
    K: Ord + Clone,
{
    fn into_grouped_ord_set(self) -> OrdSet<K> {
        let mut map: OrdSet<K> = OrdSet::new();

        for item in self {
            map.insert(item);
        }

        map
    }
}
