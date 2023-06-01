use std::fmt::Display;

use enum_map::Enum;

use crate::attribute::types::{MonsterType, Alignment};


#[derive(PartialEq, Eq, Clone, Copy, Enum, Debug)]
pub enum AttackingTarget {
    Tripped,
    MonsterType(MonsterType),
    Alignment(Alignment),
}

impl Display for AttackingTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttackingTarget::Tripped => write!(f, "Tripped Target"),
            AttackingTarget::MonsterType(monster_type) => monster_type.fmt(f),
            AttackingTarget::Alignment(alignment) => alignment.fmt(f),
        }
    }
}
