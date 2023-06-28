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
    fn iterator_into_set() {
        let items = vec![TestEnum::A, TestEnum::B, TestEnum::B, TestEnum::D];

        let mut set = items.into_iter().into_ord_set();

        assert!(set.contains(&TestEnum::A));

        assert!(set.remove(&TestEnum::B).is_some());
        assert!(!set.contains(&TestEnum::B));
        assert!(!set.contains(&TestEnum::C));
    }
}
