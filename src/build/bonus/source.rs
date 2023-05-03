use crate::build::{attribute::Attribute, feat::Feat};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Source {
    Attribute(Attribute),
    Feat(Feat),
    Base
}
