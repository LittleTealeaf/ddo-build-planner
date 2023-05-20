use builder_core::{bonus::{BonusSource, BonusType}, attribute::Attribute};

fn main() {
    let size = std::mem::size_of::<BonusSource>();
    println!("{}", size);
}
