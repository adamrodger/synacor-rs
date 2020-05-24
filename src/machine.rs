use crate::{errors::SynacorError, instruction::Instruction, memory::MemoryExt};
use std::path::Path;

/// Synacor Virtual Machine
pub struct VirtualMachine {
    memory: Vec<u16>,
    pointer: usize,
    stack: Vec<u16>,
}

impl VirtualMachine {
    /// Create a new virtual machine with memory parsed from the given file path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, SynacorError> {
        let buffer: Vec<u8> = std::fs::read(path)?;

        let mut memory: Vec<u16> = buffer
            .chunks_exact(2)
            .map(|chunk| ((chunk[1] as u16) << 8) + (chunk[0] as u16))
            .collect();

        memory.resize(std::u16::MAX as usize, 0);

        Ok(VirtualMachine { memory, pointer: 0, stack: Vec::new() })
    }

    /// execute the program until it ends
    pub fn execute(&mut self) -> Result<(), SynacorError> {
        loop {
            let instruction = &Instruction::from_state(self.pointer, &self.memory)?;

            match instruction {
                Instruction::Noop => {}
                Instruction::Halt => return Ok(()),
                Instruction::Output(arg) => {
                    print!("{}", self.memory.read(arg) as u8 as char);
                }
                Instruction::Add(dest, left, right) => {
                    let value = (self.memory.read(left) + self.memory.read(right)) % 32768u16;
                    self.memory.write(dest, value)?;
                }
                Instruction::Set(register, arg) => {
                    let value = self.memory.read(arg);
                    self.memory.write(register, value)?;
                }
                Instruction::Equal(dest, left, right) => {
                    let value = if self.memory.read(left) == self.memory.read(right) {
                        1
                    } else {
                        0
                    };
                    self.memory.write(dest, value)?;
                }
                Instruction::GreaterThan(dest, left, right) => {
                    let value = if self.memory.read(left) > self.memory.read(right) {
                        1
                    } else {
                        0
                    };
                    self.memory.write(dest, value)?;
                }
                Instruction::Jump(arg) => {
                    self.pointer = self.memory.read(arg) as usize;
                    continue;
                }
                Instruction::JumpNonZero(arg, dest) => {
                    if self.memory.read(arg) != 0 {
                        self.pointer = self.memory.read(dest) as usize;
                        continue;
                    }
                }
                Instruction::JumpZero(arg, dest) => {
                    if self.memory.read(arg) == 0 {
                        self.pointer = self.memory.read(dest) as usize;
                        continue;
                    }
                }
                Instruction::Push(arg) => {
                    let value = self.memory.read(arg);
                    self.stack.push(value);
                }
                Instruction::Pop(arg) => {
                    let value = self.stack.pop().ok_or(SynacorError::EmptyStack)?;
                    self.memory.write(arg, value)?;
                }
            }

            self.pointer += instruction.size();
        }
    }
}
