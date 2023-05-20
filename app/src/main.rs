use builder_core::{bonus::BonusSource, feat::Tome, compiler::AttributeCompiler};
use enum_map::Enum;

fn main() {
    let value = BonusSource::Feat(builder_core::feat::Feat::Tome(Tome::SpellPower));
    println!("{}", value.into_usize());

    let size = std::mem::size_of_val(&AttributeCompiler::new());

    println!("Size of Attribute Compiler: {}", size);

    let size = std::mem::size_of::<AttributeCompiler>();

    println!("{}", size);
}
