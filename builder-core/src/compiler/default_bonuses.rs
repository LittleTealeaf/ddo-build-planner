use crate::{
    attribute::types::{Ability, ArmorClass},
    bonus::{Bonus, BonusSource, BonusType},
};

const BASE: BonusSource = BonusSource::Base;
const STACKING: BonusType = BonusType::Stacking;

pub fn build_default_values() -> Vec<Bonus> {
    vec![
        Bonus::new(ArmorClass::ArmorScalar.into(), STACKING, 1f32.into(), BASE, None),
        Bonus::new(ArmorClass::ShieldScalar.into(), STACKING, 1f32.into(), BASE, None),
        Bonus::new(Ability::All.into(), STACKING, 8f32.into(), BASE, None),
    ]
}

#[cfg(test)]
mod tests {
    use crate::compiler::Compiler;

    use super::*;

    #[test]
    fn default_abilities() {
        let compiler = Compiler::default();

        for ability in Ability::VALUES {
            assert_eq!(compiler.get_attribute(&ability.into()), 8f32);
        }
    }
}
