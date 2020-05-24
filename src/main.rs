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

/// Program instruction
enum Instruction {
    Halt,
    Noop,
    Output(Argument)
}

impl Instruction {
    
    /// load an instruction from memory starting at the current pointer
    fn from_state(pointer: usize, memory: &[u16]) -> Result<Instruction, SynacorError> {
        let opcode = memory[pointer];

        match opcode {
            0 => Ok(Instruction::Halt),
            19 => Ok(Instruction::Output(memory[pointer + 1].try_into()?)),
            21 => Ok(Instruction::Noop),
            _ => Err(SynacorError::UnsupportedOpCode(opcode))
        }
    }
}

/// Argument type
#[derive(Debug)]
enum Argument {
    Literal(u16),
    Register(usize)
}

impl Argument {
    fn read(&self, memory: &[u16]) -> u16 {
        match *self {
            Argument::Literal(n) => n,
            Argument::Register(n) => memory[n]
        }
    }
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