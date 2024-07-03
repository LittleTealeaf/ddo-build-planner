use super::MapLike;

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

impl<K, V, M> MapGetOr<K, V> for M
where
    M: MapLike<K, V>,
    K: Clone,
{
    fn get_or_else<F>(&mut self, key: &K, if_empty: F) -> &V
    where
        F: FnOnce() -> V,
    {
        if !self.contains_key(key) {
            self.insert(key.clone(), if_empty());
        }
        self.get(key).expect("Expect Return Value")
    }

    fn get_mut_or_else<F>(&mut self, key: &K, if_empty: F) -> &mut V
    where
        F: FnOnce() -> V,
    {
        if !self.contains_key(key) {
            self.insert(key.clone(), if_empty());
        }
        self.get_mut(key).expect("Expect Return Value")
    }
}
