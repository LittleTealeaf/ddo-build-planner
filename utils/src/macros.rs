//! Some helpful macros

/// Repeats a given function tree-like on a list of items.
/// The function must output the same data type that it consumes
#[macro_export]
macro_rules! tree_repeat {
    ($fun: expr, $first: expr, $second: expr) => {
        $fun($first, $second)
    };

    ($fun: expr, $first: expr, $second: expr, $($rest: expr),+ $(,)?) => {
        $crate::tree_repeat!($fun, $($rest ,)+ $fun($first, $second))
    }
}
