use super::Attribute;

/// Indicates that this type can be directly converted to an attribute
pub trait ToAttribute {
    /// Converts this type directly into an attribute
    fn to_attribute(self) -> Attribute;
}

impl<T> From<T> for Attribute
where
    T: ToAttribute,
{
    fn from(value: T) -> Self {
        value.to_attribute()
    }
}
