use strum::VariantArray;

pub trait IterValues {
    fn values() -> impl Iterator<Item = Self>;
}

impl<T> IterValues for T
where
    T: VariantArray + Copy,
{
    fn values() -> impl Iterator<Item = Self> {
        Self::VARIANTS.iter().copied()
    }
}
