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
    I: IntoIterator<Item = (K, V)>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Ord, PartialOrd, PartialEq, Eq)]
    enum TestEnum {
        A,
        B,
        C,
        D,
        E,
        F,
    }

    #[test]
    fn iters_into_grouped_map() {
        let values = vec![
            (TestEnum::A, 1),
            (TestEnum::B, 2),
            (TestEnum::C, 3),
            (TestEnum::C, 4),
            (TestEnum::D, 5),
            (TestEnum::D, 5),
        ];

        let map = values.into_iter().into_grouped_ord_map();

        assert!(map.get(&TestEnum::A).unwrap().contains(&1));
        assert!(map.get(&TestEnum::B).unwrap().contains(&2));
        assert!(map.get(&TestEnum::C).unwrap().contains(&3));
        assert!(map.get(&TestEnum::C).unwrap().contains(&4));

        let items = map.get(&TestEnum::D).unwrap();
        let mut iter = items.into_iter();
        assert_eq!(Some(&5), iter.next());
        assert_eq!(Some(&5), iter.next());
        assert_eq!(None, iter.next());

        assert!(map.get(&TestEnum::F).is_none());
    }

    #[test]
    fn vecs_into_grouped_map() {
        let values = vec![
            (TestEnum::A, 1),
            (TestEnum::B, 2),
            (TestEnum::C, 3),
            (TestEnum::C, 4),
            (TestEnum::D, 5),
            (TestEnum::D, 5),
        ];

        let map = values.into_grouped_ord_map();

        assert!(map.get(&TestEnum::A).unwrap().contains(&1));
        assert!(map.get(&TestEnum::B).unwrap().contains(&2));
        assert!(map.get(&TestEnum::C).unwrap().contains(&3));
        assert!(map.get(&TestEnum::C).unwrap().contains(&4));

        let items = map.get(&TestEnum::D).unwrap();
        let mut iter = items.into_iter();
        assert_eq!(Some(&5), iter.next());
        assert_eq!(Some(&5), iter.next());
        assert_eq!(None, iter.next());

        assert!(map.get(&TestEnum::F).is_none());
    }
}
