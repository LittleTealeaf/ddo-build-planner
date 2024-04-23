use std::collections::HashSet;

use crate::attribute::Attribute;

/// Indicates that this type can have some attribute dependencies
pub trait AttributeDependencies {
    /// Checks if a given attribute is a dependencies of this object
    fn has_attr_dependency(&self, attribute: &Attribute) -> bool;

    /// Collects dependencies into an `OrdSet`
    fn include_attr_dependency(&self, set: &mut HashSet<Attribute>);

    /// Creates an ord set for dependencies
    fn get_attr_dependencies(&self) -> HashSet<Attribute> {
        let mut set = HashSet::new();
        self.include_attr_dependency(&mut set);
        set
    }
}
