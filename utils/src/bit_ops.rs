//! Operator Iterators for bitwise

use core::ops::{BitAnd, BitOr};

use itertools::Itertools;

/// Adds the ability for an `all` operator to be applied on a collection of items that implement
/// the & operator (as a &&)
pub trait BitAll<T> {
    /// Returns the bit-wise all for elements in this structure
    fn bit_all(self) -> Option<T>;
}

impl<I, T> BitAll<T> for I
where
    I: IntoIterator<Item = T>,
    T: BitAnd<T, Output = T>,
{
    fn bit_all(self) -> Option<T> {
        self.into_iter().tree_fold1(|a, b| a & b)
    }
}

/// Adds the ability for an `any` opperator to be applied on a collection of items that implement
/// the | operator (as a ||)
pub trait BitAny<T> {
    /// Returns the bit-wise any operator for elements in this structure
    fn bit_any(self) -> Option<T>;
}

impl<I, T> BitAny<T> for I
where
    I: IntoIterator<Item = T>,
    T: BitOr<T, Output = T>,
{
    fn bit_any(self) -> Option<T> {
        self.into_iter().tree_fold1(|a, b| a | b)
    }
}
