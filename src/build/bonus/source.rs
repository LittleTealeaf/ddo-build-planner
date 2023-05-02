use crate::build::{attribute::Attribute, feat::Feat};

pub enum Source {
    Attribute(Attribute),
    Feat(Feat),
}
