use builder_core::{bonus::BonusSource, compiler::AttributeCompiler, feat::Tome};
use enum_map::Enum;

fn main() {
    let value = BonusSource::Feat(builder_core::feat::Feat::Tome(Tome::SpellPower));
    println!("Source Value of Spell Power Tome: {}", value.into_usize());

    let size = std::mem::size_of_val(&AttributeCompiler::new());

    println!("Size of Attribute Compiler: {} bytes", size);
}
