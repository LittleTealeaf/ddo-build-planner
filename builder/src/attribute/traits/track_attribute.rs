// TODO: Remove TrackedAttribute?

/// Implements the check of whether a particular attribute is actually tracked.
///
/// Most likely, this
/// will be `true`, but there are some bonuses that should not be tracked and therefore should be
/// ignored.
pub trait TrackAttribute {
    /// Checks whether or not the object should be tracked.
    ///
    /// If the object should be tracked, returns `true`, otherwise returns `false`
    fn is_tracked(&self) -> bool;
}
