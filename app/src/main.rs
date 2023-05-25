use builder_core::compiler::AttributeCompiler;

fn main() {
    let size = std::mem::size_of_val(&AttributeCompiler::new());

    println!("Size of Attribute Compiler: {} bytes", size);
}
