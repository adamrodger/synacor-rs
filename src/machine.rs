use std::path::Path;
use crate::{instruction::Instruction, errors::SynacorError};

/// Synacor Virtual Machine
pub struct VirtualMachine {
    memory: Vec<u16>,
    pointer: usize
}

impl VirtualMachine {
    /// Create a new virtual machine with memory parsed from the given file path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, SynacorError> {
        let buffer = std::fs::read(path)?;

        let memory = buffer
            .chunks_exact(2)
            .map(|chunk| ((chunk[1] as u16) << 8) + (chunk[0] as u16))
            .collect::<Vec<u16>>();

        Ok(VirtualMachine {
            memory,
            pointer: 0,
        })
    }

    /// execute the program until it ends
    pub fn execute(&mut self) -> Result<(), SynacorError> {
        loop {
            let instruction = Instruction::from_state(self.pointer, &self.memory)?;

            match instruction {
                Instruction::Output(arg) => {
                    print!("{}", arg.read(&self.memory) as u8 as char);
                    self.pointer += 2;
                },
                Instruction::Noop => {
                    self.pointer += 1;
                },
                Instruction::Halt => return Ok(())
            }
        }
    }
}