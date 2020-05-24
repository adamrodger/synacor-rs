use std::{path::Path, convert::{TryInto, TryFrom}};
use errors::SynacorError;

mod errors;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("spec").join("challenge.bin");
    let mut vm = VirtualMachine::from_file(path)?;
    vm.execute()?;

    Ok(())
}

/// Synacor Virtual Machine
struct VirtualMachine {
    memory: Vec<u16>,
    pointer: usize
}

impl VirtualMachine {
    /// Create a new virtual machine with memory parsed from the given file path
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, SynacorError> {
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
    fn execute(&mut self) -> Result<(), SynacorError> {
        loop {
            let opcode: OpCode = self.memory[self.pointer].try_into()?;

            match opcode {
                OpCode::Output => {
                    let argument: Argument = self.memory[self.pointer + 1].try_into()?;

                    let c: char = match argument {
                        Argument::Literal(n) => n as u8 as char,
                        Argument::Register(n) => self.memory[n as usize] as u8 as char
                    };

                    print!("{}", c);
                    self.pointer += 2;
                },
                OpCode::Noop => {
                    self.pointer += 1;
                },
                OpCode::Halt => return Ok(())
            }
        }
    }
}

#[derive(Debug)]
enum OpCode {
    Halt,
    Noop,
    Output,
}

impl TryFrom<u16> for OpCode {
    type Error = SynacorError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Halt),
            19 => Ok(OpCode::Output),
            21 => Ok(OpCode::Noop),
            _ => Err(SynacorError::UnsupportedOpCode(value))
        }
    }
}

/// Argument type
#[derive(Debug)]
enum Argument {
    Literal(u16),
    Register(usize)
}

impl TryFrom<u16> for Argument {
    type Error = SynacorError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0..=32767 => Ok(Argument::Literal(value)),
            32768..=32775 => Ok(Argument::Register(value as usize)),
            _ => Err(SynacorError::InvalidArgument(value))
        }
    }
}