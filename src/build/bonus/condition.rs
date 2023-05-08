use crate::build::attribute::Flag;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum Condition {
    Flag(Flag),
    NoFlag(Flag),
}

impl Condition {
    pub fn into_vec(self) -> Vec<Condition> {
        vec![self]
    }
}
