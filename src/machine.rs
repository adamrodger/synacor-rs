use crate::{
    argument::Argument, errors::SynacorError, instruction::Instruction, memory::MemoryExt,
};
use io::Write;
use std::{collections::VecDeque, io, path::Path};

const MAX_MEMORY: usize = 1 << 15;
const REGISTERS: usize = 8;

/// Synacor Virtual Machine
pub struct VirtualMachine {
    memory: Vec<u16>,
    pointer: usize,
    stack: Vec<u16>,
    stdin: VecDeque<char>,
}

impl VirtualMachine {
    /// Create a new virtual machine with memory parsed from the given file path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, SynacorError> {
        let buffer: Vec<u8> = std::fs::read(path)?;

        let mut memory: Vec<u16> = buffer
            .chunks_exact(2)
            .map(|chunk| ((chunk[1] as u16) << 8) + (chunk[0] as u16))
            .collect();

        memory.resize(MAX_MEMORY + REGISTERS, 0);

        Ok(VirtualMachine {
            memory,
            pointer: 0,
            stack: Vec::with_capacity(100),
            stdin: VecDeque::with_capacity(100),
        })
    }

    /// execute the program until it ends
    pub fn execute(&mut self) -> Result<(), SynacorError> {
        loop {
            let instruction = &Instruction::from_state(self.pointer, &self.memory)?;

            match instruction {
                Instruction::Noop => {}
                Instruction::Halt => return Ok(()),
                Instruction::Input(dest) => {
                    if self.stdin.is_empty() {
                        // make sure there's no unprinted output
                        io::stdout().lock().flush()?;

                        let mut line = String::new();
                        io::stdin().read_line(&mut line)?;

                        for c in line.chars().filter(|f| *f != '\r') {
                            self.stdin.push_back(c);
                        }
                    }

                    let value = self.stdin.pop_front().ok_or(SynacorError::NoInput)?;
                    self.memory.write(dest, value as u16)?;
                }
                Instruction::Output(arg) => {
                    print!("{}", self.memory.read(arg) as u8 as char);
                }
                Instruction::Add(dest, left, right) => {
                    let value = (self.memory.read(left) + self.memory.read(right)) % 32768u16;
                    self.memory.write(dest, value)?;
                }
                Instruction::Multiply(dest, left, right) => {
                    let value = ((self.memory.read(left) as u32 * self.memory.read(right) as u32)
                        % 32768u32) as u16;
                    self.memory.write(dest, value)?;
                }
                Instruction::Mod(dest, left, right) => {
                    let value = self.memory.read(left) % self.memory.read(right);
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
                Instruction::And(dest, left, right) => {
                    let value = self.memory.read(left) & self.memory.read(right);
                    self.memory.write(dest, value)?;
                }
                Instruction::Or(dest, left, right) => {
                    let value = self.memory.read(left) | self.memory.read(right);
                    self.memory.write(dest, value)?;
                }
                Instruction::Not(dest, arg) => {
                    let value = !self.memory.read(arg) & 0x7FFF; // 15-bit bitwise inverse
                    self.memory.write(dest, value)?;
                }
                Instruction::Call(arg) => {
                    self.stack.push((self.pointer + instruction.size()) as u16);

                    let value = self.memory.read(arg);
                    self.pointer = value as usize;
                    continue;
                }
                Instruction::Return => {
                    if let Some(value) = self.stack.pop() {
                        self.pointer = value as usize;
                        continue;
                    } else {
                        return Ok(());
                    }
                }
                Instruction::Read(dest, src) => {
                    let src_ref = Argument::Reference(self.memory.read(src));
                    let value = self.memory.read(&src_ref);
                    self.memory.write(dest, value)?;
                }
                Instruction::Write(dest, src) => {
                    let dest_ref = Argument::Reference(self.memory.read(dest)); // de-ref
                    let value = self.memory.read(src);
                    self.memory.write(&dest_ref, value)?;
                }
            }

            self.pointer += instruction.size();
        }
    }
}
