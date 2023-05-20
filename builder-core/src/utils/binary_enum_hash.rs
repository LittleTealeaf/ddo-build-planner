// An enum Hash Map using binary search

use enum_map::Enum;

pub struct EnumBinaryMap<K: Enum + Copy, V> {
    array: Vec<(K, V)>,
}

impl<K: Enum + Copy, V> EnumBinaryMap<K, V> {
    #[inline]
    pub fn new() -> Self {
        Self {
            array: Vec::default(),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self
            .array
            .binary_search_by_key(&key.into_usize(), |(k, _)| k.into_usize())
            .ok()?;

        let (_, value) = self.array.get(index)?;

        Some(value)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let index = self
            .array
            .binary_search_by_key(&key.into_usize(), |(k, _)| k.into_usize())
            .ok()?;

        let (_, value) = self.array.get_mut(index)?;

        Some(value)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self
            .array
            .binary_search_by_key(&key.into_usize(), |(k, _)| k.into_usize());

        match index {
            Ok(index) => {
                let item = self.array.remove(index);
                self.array.insert(index, (key, value));
                Some(item.1)
            }
            Err(index) => {
                self.array.insert(index, (key, value));
                None
            }
        }
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
}
