use crate::{argument::Argument, errors::SynacorError};
use std::convert::TryInto;

/// Program instruction
pub enum Instruction {
    Halt,
    Jump(Argument),
    JumpNonZero(Argument, Argument),
    JumpZero(Argument, Argument),
    Noop,
    Output(Argument),
    Set(Argument, Argument),
}

impl Instruction {
    /// load an instruction from memory starting at the current pointer
    pub fn from_state(pointer: usize, memory: &[u16]) -> Result<Instruction, SynacorError> {
        let opcode = memory[pointer];

        match opcode {
            0 => Ok(Instruction::Halt),
            1 => Ok(Instruction::Set(
                memory[pointer + 1].try_into()?,
                memory[pointer + 2].try_into()?,
            )),
            6 => Ok(Instruction::Jump(memory[pointer + 1].try_into()?)),
            7 => Ok(Instruction::JumpNonZero(
                memory[pointer + 1].try_into()?,
                memory[pointer + 2].try_into()?,
            )),
            8 => Ok(Instruction::JumpZero(
                memory[pointer + 1].try_into()?,
                memory[pointer + 2].try_into()?,
            )),
            19 => Ok(Instruction::Output(memory[pointer + 1].try_into()?)),
            21 => Ok(Instruction::Noop),
            _ => Err(SynacorError::UnsupportedOpCode(opcode)),
        }
    }

    /// get the size of the instruction
    pub fn size(&self) -> usize {
        match self {
            Instruction::Halt => 1,
            Instruction::Jump(_) => 2,
            Instruction::JumpNonZero(_, _) => 3,
            Instruction::JumpZero(_, _) => 3,
            Instruction::Noop => 1,
            Instruction::Output(_) => 2,
            Instruction::Set(_, _) => 3,
        }
    }
}
