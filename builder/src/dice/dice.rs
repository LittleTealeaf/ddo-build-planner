
pub enum Dice {
    Value(f32),
    Roll {
        count: f32,
        sides: u32
    },
    Sum(Vec<Dice>),
    Product(Vec<Dice>),
}
