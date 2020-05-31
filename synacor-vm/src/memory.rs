use crate::{argument::Argument, errors::SynacorError};

pub type Memory = Vec<u16>;

pub trait MemoryExt {
    /// read from memory with the given argument
    fn read(&self, src: &Argument) -> u16;

    /// write to memory with the given value
    fn write(&mut self, dest: &Argument, value: u16) -> Result<(), SynacorError>;
}

impl MemoryExt for Memory {
    fn read(&self, src: &Argument) -> u16 {
        match *src {
            Argument::Literal(n) => n,
            Argument::Register(n) => self[n],
            Argument::Reference(n) => self[n as usize],
        }
    }

    fn write(&mut self, dest: &Argument, value: u16) -> Result<(), SynacorError> {
        match *dest {
            Argument::Literal(n) => Err(SynacorError::WriteToLiteral(n)),
            Argument::Register(n) => {
                self[n] = value;
                Ok(())
            }
            Argument::Reference(n) => {
                self[n as usize] = value;
                Ok(())
            }
        }
    }
}
