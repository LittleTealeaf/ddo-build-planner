use im::OrdMap;

/// Provides the functionality of converting a type into a grouped map, or a map that takes a key
/// and lists all values that appeared with that key.
pub trait IntoOrdGroupMap<K, V>
where
    K: Ord,
{
    /// Converts into a grouped map, listing the key and all corresponding values under that key.
    fn into_grouped_ord_map(self) -> OrdMap<K, Vec<V>>;
}

impl<I, K, V> IntoOrdGroupMap<K, V> for I
where
    I: Iterator<Item = (K, V)>,
    K: Ord + Clone,
    V: Clone,
{
    fn into_grouped_ord_map(self) -> OrdMap<K, Vec<V>> {
        let mut map: OrdMap<K, Vec<V>> = OrdMap::new();

        for (key, value) in self {
            if let Some(set) = map.get_mut(&key) {
                set.push(value);
            } else {
                map.insert(key, vec![value]);
            }
        }

        map
    }
}
