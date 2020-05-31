extern crate synacor_vm;

use synacor_vm::machine::VirtualMachine;
use std::{io::Write, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("spec")
        .join("challenge.bin");
    let mut vm = VirtualMachine::from_file(path)?;

    // connect to input/output events
    vm.on_output(write_stdout_line);
    vm.on_input_required(read_stdin_line);

    vm.execute()?;

    Ok(())
}

fn read_stdin_line() -> String {
    // make sure there's no unprinted output
    std::io::stdout()
        .lock()
        .flush()
        .expect("Failed to flush stdout");

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read from stdin");

    line
}

fn write_stdout_line(line: String) {
    println!("{}", &line);
}
