use builder_core::{
    attribute::{Ability, Attribute, FiligreeSet},
    bonus::{Bonus, BonusSource, BonusType},
    breakdown::Breakdowns,
};

fn main() {
    let mut breakdown = Breakdowns::new();

    breakdown.insert_bonuses(vec![
        Bonus::new(
            Attribute::Ability(Ability::Strength),
            BonusType::Stacking,
            50f32,
            BonusSource::Unique(1),
            None,
        ),
        Bonus::new(
            Attribute::FiligreeSet(FiligreeSet::NystulsMysticalDefense),
            BonusType::Stacking,
            4f32,
            BonusSource::Unique(1),
            None,
        ),
        Bonus::new(
            Attribute::FiligreeSet(FiligreeSet::TrappersDelight),
            BonusType::Stacking,
            3f32,
            BonusSource::Unique(1),
            None,
        ),
        Bonus::new(
            Attribute::FiligreeSet(FiligreeSet::NextFall),
            BonusType::Stacking,
            4f32,
            BonusSource::Unique(1),
            None,
        ),
        Bonus::new(
            Attribute::FiligreeSet(FiligreeSet::ShatteredDevice),
            BonusType::Stacking,
            4f32,
            BonusSource::Unique(1),
            None,
        ),
        Bonus::new(
            Attribute::FiligreeSet(FiligreeSet::LunarMagic),
            BonusType::Stacking,
            5f32,
            BonusSource::Unique(1),
            None,
        ),
        Bonus::new(
            Attribute::FiligreeSet(FiligreeSet::Divinity),
            BonusType::Stacking,
            5f32,
            BonusSource::Unique(1),
            None,
        ),
    ]);

    let ser = ron::to_string(&breakdown).unwrap();

    println!("{}", ser);

    let mut new_breakdowns: Breakdowns = ron::from_str(&ser).unwrap();

    let values = new_breakdowns.get_all_attributes();

    println!();

    values.iter().for_each(|(key, value)| {
        println!("{}: {}", key.to_string(), value);
    });
}
