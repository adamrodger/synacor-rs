extern crate synacor_vm;

mod location;

use crate::location::{Item, Location};
use std::collections::HashMap;
use std::path::Path;
use synacor_vm::machine::{VirtualMachine, YieldReason};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("spec")
        .join("challenge.bin");
    let mut vm = VirtualMachine::from_file(path)?;
    let mut maze = HashMap::new();

    loop {
        let reason = vm.execute()?;
        let output = vm.flush_stdout();

        print!("{}", output);

        let location: Location = output.parse()?;
        let id = location.id().clone();

        maze.entry(id).or_insert(location);

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

// player
struct Player<'a> {
    location: &'a Location,
    inventory: Vec<&'a Item>,
}
