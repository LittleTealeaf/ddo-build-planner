use core::hash::{BuildHasher, Hash};
use std::collections;

use im::OrdMap;

/// Defines some type as being map-like. This allows for each of the methods to work for any types
/// that implement this trait
pub trait MapLike<K, V> {
    fn contains_key(&self, key: &K) -> bool;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
}

impl<K, V, S> MapLike<K, V> for collections::HashMap<K, V, S>
where
    K: Hash + Eq + PartialEq + Clone,
    S: BuildHasher,
{
    fn contains_key(&self, key: &K) -> bool {
        self.contains_key(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut(key)
    }
}

impl<K, V, S> MapLike<K, V> for im::HashMap<K, V, S>
where
    K: Hash + Eq + PartialEq + Clone,
    V: Clone,
    S: BuildHasher,
{
    fn contains_key(&self, key: &K) -> bool {
        self.contains_key(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut(key)
    }
}

impl<K, V> MapLike<K, V> for OrdMap<K, V>
where
    K: Ord + PartialOrd + Clone,
    V: Clone,
{
    fn contains_key(&self, key: &K) -> bool {
        self.contains_key(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut(key)
    }
}
