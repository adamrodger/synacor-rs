use crate::{argument::Argument, errors::SynacorError};

pub type Memory = Vec<u16>;

pub trait MemoryExt {
    /// read from memory with the given argument
    fn read(&self, argument: &Argument) -> u16;

    /// write to memory with the given argument
    fn write(&mut self, argument: &Argument, value: u16) -> Result<(), SynacorError>;
}

impl MemoryExt for Memory {
    fn read(&self, argument: &Argument) -> u16 {
        match *argument {
            Argument::Literal(n) => n,
            Argument::Register(n) => self[n as usize],
        }
    }

    fn write(&mut self, argument: &Argument, value: u16) -> Result<(), SynacorError> {
        match *argument {
            Argument::Literal(n) => Err(SynacorError::ReadOnly(n)),
            Argument::Register(n) => {
                self[n] = value;
                Ok(())
            }
        }
    }
}
