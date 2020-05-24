use crate::{argument::Argument, errors::SynacorError};
use std::convert::TryInto;

/// Program instruction
pub enum Instruction {
    Halt,
    Noop,
    Output(Argument),
}

impl Instruction {
    /// load an instruction from memory starting at the current pointer
    pub fn from_state(pointer: usize, memory: &[u16]) -> Result<Instruction, SynacorError> {
        let opcode = memory[pointer];

        match opcode {
            0 => Ok(Instruction::Halt),
            19 => Ok(Instruction::Output(memory[pointer + 1].try_into()?)),
            21 => Ok(Instruction::Noop),
            _ => Err(SynacorError::UnsupportedOpCode(opcode)),
        }
    }
}
