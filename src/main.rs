use machine::VirtualMachine;
use std::path::Path;

mod argument;
mod errors;
mod instruction;
mod machine;
mod memory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("spec")
        .join("challenge.bin");
    let mut vm = VirtualMachine::from_file(path)?;
    vm.execute()?;

    Ok(())
}
