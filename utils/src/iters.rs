//! Provides utilities for dealing with iterators

#[macro_export]
/// Provides the same implementation of `chain!()`, but constructs the iterator into a tree-like
/// format.
///
/// DOES NOT RESPECT ORDER
macro_rules! chain_tree {

    (+) => {
        core::iter::empty()
    };

    (+$single:expr) => {
        $single
    };

    (+$first:expr, $second:expr) => {
        core::iter::Iterator::chain(
            $first,
            $second,
        )
    };

    (+$first:expr, $second:expr  $(, $rest:expr )+ $(,)?) => {
        chain_tree!(+ $($rest ,)+ core::iter::Iterator::chain(
            $first,
            $second,
        ))
    };

    () => {
        chain_tree!(+)
    };

    ($first: expr $(, $item:expr)+ $(,)?) => {
        chain_tree!(+ core::iter::IntoIterator::into_iter($first) $(, core::iter::IntoIterator::into_iter($item))+ )
    }
}
