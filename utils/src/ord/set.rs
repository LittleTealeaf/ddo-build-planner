use im::OrdSet;

/// Provides the ability to convert into a set.
pub trait IntoOrdSet<K>
where
    K: Ord,
{
    /// Converts into a set using the [`Ord`] trait.
    fn into_ord_set(self) -> OrdSet<K>;
}

impl<I, K> IntoOrdSet<K> for I
where
    I: Iterator<Item = K>,
    K: Ord + Clone,
{
    fn into_ord_set(self) -> OrdSet<K> {
        let mut set = OrdSet::new();

        for item in self {
            set.insert(item);
        }

        set
    }
}
