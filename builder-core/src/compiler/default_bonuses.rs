use crate::{
    attribute::types::{Ability, ArmorClass},
    bonus::{Bonus, BonusSource, BonusType},
};

const BASE: BonusSource = BonusSource::Base;
const STACKING: BonusType = BonusType::Stacking;

pub fn build_default_values() -> Vec<Bonus> {
    vec![
        Bonus::new(
            ArmorClass::ArmorScalar.into(),
            STACKING,
            1f32.into(),
            BASE,
            None,
        ),
        Bonus::new(
            ArmorClass::ShieldScalar.into(),
            STACKING,
            1f32.into(),
            BASE,
            None,
        ),
        Bonus::new(ArmorClass::Scalar.into(), STACKING, 1f32.into(), BASE, None),
        Bonus::new(Ability::All.into(), STACKING, 8f32.into(), BASE, None),
    ]
}

#[cfg(test)]
mod tests {
    use crate::compiler::Compiler;

    use super::*;

    #[test]
    fn abilities() {
        let mut compiler = Compiler::default();

        for ability in Ability::VALUES {
            assert_eq!(compiler.get_attribute(&ability.into()), 8f32);
        }
    }

    #[test]
    fn ac_scalars() {
        let mut compiler = Compiler::default();

        assert_eq!(
            compiler.get_attribute(&ArmorClass::ShieldScalar.into()),
            1f32
        );
        assert_eq!(
            compiler.get_attribute(&ArmorClass::ArmorScalar.into()),
            1f32
        );
        assert_eq!(compiler.get_attribute(&ArmorClass::Scalar.into()), 1f32);
    }
}
