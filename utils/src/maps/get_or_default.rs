use super::MapGetOr;

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
