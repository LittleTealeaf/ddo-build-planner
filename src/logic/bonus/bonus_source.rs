use crate::logic::{attribute::Attribute, feat::Feat};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum BonusSource {
    Attribute(Attribute),
    Feat(Feat),
    Unique(usize),
}
