// An enum Hash Map using binary search

use std::iter::Map;
use std::marker::PhantomData;
use std::vec::IntoIter;

use enum_map::Enum;

/// An Enum-based Hash Map that utilises binary search instead of Hashing or an extensive array.
///
/// While [EnumMap](enum_map::EnumMap) does a good job at avoiding the complexity overhead of hashing enums, it doesn't do a good job at storage size for large enums. In this crate, some of the enums can be, and many times are, hundreds to thousands of possible states long. If [EnumMap](enum_map::EnumMap) is used directly, it would result in an array of hundreds to thousands of values that sometimes will be untouched. This structure is similar to an [EnumMap](enum_map::EnumMap), except it uses a [Vec] to store data, adding only values that are inserted to preserve space.
pub struct EnumBinaryMap<K: Enum + Copy, V> {
    array: Vec<(usize, V)>,
    enum_type: PhantomData<K>,
}

impl<K: Enum + Copy, V> Default for EnumBinaryMap<K, V> {
    fn default() -> Self {
        Self {
            array: Vec::new(),
            enum_type: PhantomData,
        }
    }
}

impl<K: Enum + Copy, V> EnumBinaryMap<K, V> {
    /// Creates a new instance of the [`EnumBinaryMap`]
    ///
    /// This uses [`Self::default()`] to create a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new instance of [`EnumBinaryMap`] with a set initial capacity
    ///
    /// This internally uses the [`Vec::with_capacity()`] method to initialize the map with a set capacity.
    pub fn with_capacity(size: usize) -> Self {
        Self {
            array: Vec::with_capacity(size),
            enum_type: PhantomData,
        }
    }

    /// Gets the value in the map from a key.
    ///
    /// If the key is present will return a [`Some`] object with a reference to the value. If the key is not present, it will return a [`None`]
    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self
            .array
            .binary_search_by_key(&key.into_usize(), |(k, _)| *k)
            .ok()?;

        let (_, value) = self.array.get(index)?;

        Some(value)
    }

    /// Gets a mutable reference to the value stored with the key.
    ///
    /// If the key is present, this will return a [`Some`] object with a mutable reference to the value. Otherwise [`None`] is returned.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let index = self
            .array
            .binary_search_by_key(&key.into_usize(), |(k, _)| *k)
            .ok()?;

        let (_, value) = self.array.get_mut(index)?;

        Some(value)
    }

    /// Inserts the value into the map with its associated key.
    ///
    /// If there is already an entry with the given key, this will return [`Some`] with the previous value. If there is no key present, a [`None`] will be returned.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let key_usize = key.into_usize();

        let index = self.array.binary_search_by_key(&key_usize, |(k, _)| *k);

        match index {
            Ok(index) => {
                let (_, item) = self.array.remove(index);
                self.array.insert(index, (key_usize, value));
                Some(item)
            }
            Err(index) => {
                self.array.insert(index, (key_usize, value));
                None
            }
        }
    }

    ///Pops a key and value pair from the map
    ///
    /// This is used when the EnumMap is utilised as a "queue".
    pub fn pop(&mut self) -> Option<(K, V)> {
        let (key, value) = self.array.pop()?;
        Some((K::from_usize(key), value))
    }

    /// Returns an iterator over the keys and values of the map
    pub fn iter(&self) -> impl Iterator<Item = (K, &V)> {
        self.array
            .iter()
            .map(|(key, value)| (K::from_usize(*key), value))
    }

    /// Shrinks the capacity down to just fit the array.
    pub fn shrink_to_fit(&mut self) {
        self.array.shrink_to_fit()
    }

    /// Checks if a key can be found in the map
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Attempts to remove a key from the map. If it successfully is removed, the associated value will be returned.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let key_usize = key.into_usize();

        let index = self
            .array
            .binary_search_by_key(&key_usize, |(k, _)| *k)
            .ok()?;

        Some(self.array.remove(index).1)
    }
}

impl<K: Enum + Copy, V> IntoIterator for EnumBinaryMap<K, V> {
    type Item = (K, V);

    type IntoIter = Map<IntoIter<(usize, V)>, fn((usize, V)) -> (K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.array
            .into_iter()
            .map(|(key, value)| (K::from_usize(key), value))
    }
}

impl<K: Enum + Copy, V> FromIterator<(K, V)> for EnumBinaryMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut array = Vec::new();
        for (key, value) in iter {
            array.push((key.into_usize(), value));
        }
        Self {
            array,
            enum_type: PhantomData,
        }
    }
}

/// Implementations when the value implements [`Default`].
impl<K: Enum + Copy, V: Default> EnumBinaryMap<K, V> {
    /// Returns a mutable reference to a value associated with the key.
    ///
    /// If there is no value associated with the key in the map, then a new entry will be created using [`Default::default()`] to initialize the value.
    pub fn get_mut_or_default(&mut self, key: &K) -> &mut V {
        let binary_result = self
            .array
            .binary_search_by_key(&key.into_usize(), |(k, _)| *k);
        match binary_result {
            Ok(index) => &mut self.array[index].1,
            Err(index) => {
                self.array.insert(index, (key.into_usize(), V::default()));
                &mut self.array[index].1
            }
        }
    }
}

impl<K: Enum + Copy, V, I: Iterator<Item = (K, V)> + Sized> From<I> for EnumBinaryMap<K, Vec<V>> {
    fn from(value: I) -> Self {
        let mut map: EnumBinaryMap<K, Vec<V>> = EnumBinaryMap::default();
        for (key, value) in value {
            map.get_mut_or_default(&key).push(value);
        }
        map
    }
}

impl<K: Enum + Copy, I: Iterator<Item = K> + Sized> From<I> for EnumBinaryMap<K, ()> {
    fn from(value: I) -> Self {
        let mut map = EnumBinaryMap::default();
        for key in value {
            map.get_mut_or_default(&key);
        }
        map
    }
}

impl<K: Enum + Copy, V> From<EnumBinaryMap<K, V>> for Vec<(K, V)> {
    fn from(value: EnumBinaryMap<K, V>) -> Self {
        value.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Enum, Clone, Copy)]
    enum TestSub {
        A,
        B,
        C,
    }

    #[derive(Enum, Clone, Copy)]
    enum Test {
        A,
        B,
        C(TestSub),
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
    }

    #[test]
    fn test_inserts() {
        let mut map = EnumBinaryMap::new();

        map.insert(Test::A, 3);

        assert_eq!(Some(&3), map.get(&Test::A));
    }

    #[test]
    fn test_multiple() {
        let mut map = EnumBinaryMap::new();

        const ITEMS: [(Test, u32); 10] = [
            (Test::A, 5),
            (Test::C(TestSub::B), 2),
            (Test::D, 3),
            (Test::E, 4),
            (Test::C(TestSub::C), 6),
            (Test::L, 7),
            (Test::H, 8),
            (Test::C(TestSub::A), 9),
            (Test::I, 10),
            (Test::F, 11),
        ];

        for item in ITEMS {
            map.insert(item.0, item.1);
        }

        for (key, value) in ITEMS {
            assert_eq!(Some(&value), map.get(&key));
        }
    }

    #[test]
    fn with_capacity_sets_capacity() {
        let map = EnumBinaryMap::<Test, ()>::with_capacity(50);
        assert_eq!(50, map.array.capacity())
    }
}
