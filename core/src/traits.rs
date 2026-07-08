pub trait IterValues {
    fn values() -> impl Iterator<Item = Self>;
}
