use crate::logic::{attribute::Attribute, feats::Feat};

pub enum Source {
    Attribute(Attribute),
    Feat(Feat),
}
