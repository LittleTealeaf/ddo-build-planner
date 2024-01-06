//! Additional utilities only used in debug

use crate::{
    attribute::Attribute,
    bonus::{BonusSource, BonusType},
};

/// Standard attribute that converts into any enum that has a debug variant
pub struct DebugValue(pub u8);

impl From<DebugValue> for Attribute {
    fn from(value: DebugValue) -> Self {
        Self::Debug(value.0)
    }
}

impl From<DebugValue> for BonusSource {
    fn from(value: DebugValue) -> Self {
        Self::Debug(value.0)
    }
}

impl From<DebugValue> for BonusType {
    fn from(value: DebugValue) -> Self {
        Self::Debug(value.0)
    }
}
