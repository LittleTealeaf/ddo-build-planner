use crate::build::attribute::{flag::Flag};

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum Condition {
    HasFlag(Flag)
}


impl Condition {
    pub fn into_vec(self) -> Vec<Condition> {
        vec![self]
    }
}
