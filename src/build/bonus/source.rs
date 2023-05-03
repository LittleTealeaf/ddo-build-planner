use crate::build::{attribute::Attribute, feat::Feat};

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum Source {
    Attribute(Attribute),
    Feat(Feat),
    Base
}
