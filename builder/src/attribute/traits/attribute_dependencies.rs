use crate::attribute::Attribute;
use im::OrdSet;

/// Indicates that this type can have some attribute dependnecies
pub trait AttributeDependencies {
    /// Checks if a given attribute is a dependdency of this object
    fn has_attr_dependency(&self, attribute: Attribute) -> bool;

    /// Collects dependencies into an `OrdSet`
    fn include_attr_dependency(&self, set: &mut OrdSet<Attribute>);

    /// Creates an ord set for dependencies
    fn get_attr_dependencies(&self) -> OrdSet<Attribute> {
        let mut set = OrdSet::new();
        self.include_attr_dependency(&mut set);
        set
    }
}
