mod common;

use ddo_core::{
    player_stats::PlayerStats,
    types::{ability::Ability, skill::Skill},
};
use rust_decimal_macros::dec;

use crate::common::player_stats::{set_ability_modifier, set_ability_score};
use ddo_core::traits::IterValues;

mod ability_score {
    use super::*;
    use test_log::test;

    #[test]
    fn base_is_8() {
        let mut stats = PlayerStats::default();
        for ability in Ability::values() {
            let value = stats.evaluate_attribute(ability.score(), None);
            assert_eq!(value, dec!(8));
        }
    }
}

mod ability_modifier {
    use test_log::test;

    use super::*;

    #[test]
    fn score_12_is_1_modifier() {
        let mut stats = PlayerStats::default();
        for ability in Ability::values() {
            set_ability_score(&mut stats, ability, dec!(12));
            assert_eq!(dec!(1), stats.evaluate_attribute(ability.modifier(), None));
        }
    }

    #[test]
    fn score_7_is_neg_2_modifier() {
        let mut stats = PlayerStats::default();
        for ability in Ability::values() {
            set_ability_score(&mut stats, ability, dec!(7));
            assert_eq!(dec!(-2), stats.evaluate_attribute(ability.modifier(), None));
        }
    }

    #[test]
    fn score_8_is_neg_1_modifier() {
        let mut stats = PlayerStats::default();
        for ability in Ability::values() {
            set_ability_score(&mut stats, ability, dec!(8));
            assert_eq!(dec!(-1), stats.evaluate_attribute(ability.modifier(), None));
        }
    }

    #[test]
    fn score_9_is_neg_1_modifier() {
        let mut stats = PlayerStats::default();
        for ability in Ability::values() {
            set_ability_score(&mut stats, ability, dec!(9));
            assert_eq!(dec!(-1), stats.evaluate_attribute(ability.modifier(), None));
        }
    }

    #[test]
    fn score_10_is_0_modifier() {
        let mut stats = PlayerStats::default();
        for ability in Ability::values() {
            set_ability_score(&mut stats, ability, dec!(10));
            assert_eq!(dec!(0), stats.evaluate_attribute(ability.modifier(), None));
        }
    }

    #[test]
    fn score_11_is_0_modifier() {
        let mut stats = PlayerStats::default();
        for ability in Ability::values() {
            set_ability_score(&mut stats, ability, dec!(11));
            assert_eq!(dec!(0), stats.evaluate_attribute(ability.modifier(), None));
        }
    }
}

mod skills {
    use super::*;
    use test_log::test;

    #[test]
    fn skill_improved_by_stat() {
        let mut stats = PlayerStats::default();
        for skill in Skill::values() {
            let ability = skill.ability();
            set_ability_modifier(&mut stats, ability, dec!(0));
            assert_eq!(dec!(0), stats.evaluate_attribute(skill, None));
            set_ability_modifier(&mut stats, ability, dec!(1));
            assert_eq!(dec!(1), stats.evaluate_attribute(skill, None));
            set_ability_modifier(&mut stats, ability, dec!(-1));
            assert_eq!(dec!(-1), stats.evaluate_attribute(skill, None));
        }
    }
}
