use builder::attribute::Attribute;
use itertools::chain;
use utils::enums::StaticOptions;

pub enum AttributeOptions {
    Attribute(Attribute),
    SetBonus,
}

impl AttributeOptions {
    pub fn options() -> impl Iterator<Item = Self> {
        chain!(
            [Self::SetBonus],
            Attribute::get_static().map(Self::Attribute)
        )
    }
}
