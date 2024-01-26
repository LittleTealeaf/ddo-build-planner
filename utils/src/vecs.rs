//! Utilities for working with Vectors

/// Provides a function to remove elements based on a perdicate
pub trait FilterRemove<T> {
    /// Removes all elements that make the current filter true
    ///
    /// Does not maintain order
    fn remove_filter(&mut self, filter: impl Fn(&T) -> bool) -> Vec<T>;
}

impl<T> FilterRemove<T> for Vec<T> {
    fn remove_filter(&mut self, filter: impl Fn(&T) -> bool) -> Self {
        let mut items = Self::new();

        for i in (0..self.len()).rev() {
            if filter(&self[i]) {
                items.push(self.swap_remove(i));
            }
        }
        items
    }
}

#[cfg(test)]
mod tests {
    use super::FilterRemove;

    #[test]
    fn remove_odd_items() {
        let mut items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let evens = items.remove_filter(|n| n % 2 == 0);

        assert!(items.into_iter().all(|n| n % 2 == 1));
        assert!(evens.into_iter().all(|n| n % 2 == 0));
    }
}
