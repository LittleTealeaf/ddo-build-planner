use crate::{attribute_subtype, build::attribute::Attribute};

attribute_subtype!(Toggle, (Blocking "Blocking"));

impl From<Toggle> for Attribute {
    fn from(value: Toggle) -> Self {
        Attribute::Toggle(value)
    }
}
