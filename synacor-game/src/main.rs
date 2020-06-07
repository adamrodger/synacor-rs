extern crate synacor_vm;

use std::path::Path;
use synacor_vm::machine::{VirtualMachine, YieldReason};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("spec")
        .join("challenge.bin");
    let mut vm = VirtualMachine::from_file(path)?;

    loop {
        let reason = vm.execute()?;
        let output = vm.flush_stdout();
        print!("{}", output);

        match reason {
            YieldReason::Halted => return Ok(()),
            YieldReason::InputRequired => {
                let mut line = String::new();
                std::io::stdin()
                    .read_line(&mut line)
                    .expect("Failed to read from stdin");

                line = line.replace("\r", "");
                vm.write_stdin(line);
            }
        }
    }
}
