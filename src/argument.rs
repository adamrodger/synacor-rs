use std::convert::TryFrom;
use crate::errors::SynacorError;

/// Argument type
#[derive(Debug)]
pub enum Argument {
    Literal(u16),
    Register(usize)
}

impl Argument {
    pub fn read(&self, memory: &[u16]) -> u16 {
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