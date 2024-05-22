//! Additional generic implementations for ``HashMaps`` to simplify code

use core::hash::{BuildHasher, Hash};
use std::collections::HashMap;

use im::OrdMap;

/// Provides functions to automatically insert a "default" value and return that value if no value
/// is found within the map.
pub trait MapGetOr<K, V> {
    /// Executes and inserts the result of a function if the key is not found within the map.
    /// Otherwise returns the key's value.
    fn get_mut_or_else<F>(&mut self, key: &K, if_empty: F) -> &mut V
    where
        F: FnOnce() -> V;

    /// Inserts a default value if the key is not found within the map
    /// Otherwise returns the key's value.
    fn get_mut_or(&mut self, key: &K, default: V) -> &mut V {
        self.get_mut_or_else(key, || default)
    }

    /// Executes and inserts the result of the function if the key is not found witin the map.
    /// Returns what is left in the map when completed
    fn get_or_else<F>(&mut self, key: &K, if_empty: F) -> &V
    where
        F: FnOnce() -> V;

    /// Inserts the value if the key is not in the map, returns the resulting value associated with
    /// said key.
    fn get_or(&mut self, key: &K, default: V) -> &V {
        self.get_or_else(key, || default)
    }
}

macro_rules! impl_get_mut_or_else {
    ($self: ident, $key: ident, $if_empty: ident) => {{
        if !$self.contains_key($key) {
            $self.insert($key.clone(), $if_empty());
        }
        $self.get_mut($key).expect("Expected Return Value")
    }};
}

macro_rules! impl_get_or_else {
    ($self: ident, $key: ident, $if_empty: ident) => {{
        if !$self.contains_key($key) {
            $self.insert($key.clone(), $if_empty());
        }
        $self.get($key).expect("Expcted Return Value")
    }};
}

impl<K, V, S> MapGetOr<K, V> for HashMap<K, V, S>
where
    K: Hash + Eq + PartialEq + Clone,
    S: BuildHasher,
{
    fn get_mut_or_else<F>(&mut self, key: &K, if_empty: F) -> &mut V
    where
        F: FnOnce() -> V,
    {
        impl_get_mut_or_else!(self, key, if_empty)
    }

    fn get_or_else<F>(&mut self, key: &K, if_empty: F) -> &V
    where
        F: FnOnce() -> V,
    {
        impl_get_or_else!(self, key, if_empty)
    }
}

impl<K, V, S> MapGetOr<K, V> for im::HashMap<K, V, S>
where
    K: Hash + Eq + PartialEq + Clone,
    V: Clone,
    S: BuildHasher,
{
    fn get_mut_or_else<F>(&mut self, key: &K, if_empty: F) -> &mut V
    where
        F: FnOnce() -> V,
    {
        impl_get_mut_or_else!(self, key, if_empty)
    }

    fn get_or_else<F>(&mut self, key: &K, if_empty: F) -> &V
    where
        F: FnOnce() -> V,
    {
        impl_get_or_else!(self, key, if_empty)
    }
}

impl<K, V> MapGetOr<K, V> for OrdMap<K, V>
where
    K: Ord + PartialOrd + Clone,
    V: Clone,
{
    fn get_mut_or_else<F>(&mut self, key: &K, if_empty: F) -> &mut V
    where
        F: FnOnce() -> V,
    {
        impl_get_mut_or_else!(self, key, if_empty)
    }

    fn get_or_else<F>(&mut self, key: &K, if_empty: F) -> &V
    where
        F: FnOnce() -> V,
    {
        impl_get_or_else!(self, key, if_empty)
    }
}

/// Extends [`MapGetMutOr`] to add a function for all value types that implement [`Default`].
pub trait MapGetOrDefault<K, V>
where
    V: Default,
{
    /// Attempts to get the resulting value for the given key. If none is foundm, inserts the
    /// result of [`Default::default()`] and returns a reference of result;
    fn get_or_default(&mut self, key: &K) -> &V;

    /// Attempts to get the resulting value for the given key. If none is found, inserts the result
    /// of [`Default::default()`] and returns a mutable reference of the result.
    fn get_mut_or_default(&mut self, key: &K) -> &mut V;
}

impl<T, K, V> MapGetOrDefault<K, V> for T
where
    T: MapGetOr<K, V>,
    V: Default,
{
    fn get_or_default(&mut self, key: &K) -> &V {
        self.get_or_else(key, Default::default)
    }

    fn get_mut_or_default(&mut self, key: &K) -> &mut V {
        self.get_mut_or_else(key, Default::default)
    }
}

/// Provides the `.into_grouped_hash_map` function for iterators
pub trait IntoGroupedHashMap<K, V> {
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
