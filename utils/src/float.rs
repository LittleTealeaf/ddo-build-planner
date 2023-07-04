//! Helper functions for comparing floats


/// Indicates that a type can be said to be equal wthin an error margin
pub trait ErrorMargin {
    /// Returns `true` if the margin of error is lower than the specified amount
    fn within_margin(&self, other: &Self) -> bool;
}

impl ErrorMargin for f32 {
    fn within_margin(&self, other: &Self) -> bool {
        (self - other).abs() < Self::EPSILON
    }
}

impl ErrorMargin for f64 {
    fn within_margin(&self, other: &Self) -> bool {
        (self - other).abs() < Self::EPSILON
    }
}
