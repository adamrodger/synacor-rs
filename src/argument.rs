use crate::errors::SynacorError;
use std::convert::TryFrom;

/// Argument type
#[derive(Debug)]
pub enum Argument {
    Literal(u16),
    Register(usize),
}

impl TryFrom<u16> for Argument {
    type Error = SynacorError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0..=32767 => Ok(Argument::Literal(value)),
            32768..=32775 => Ok(Argument::Register(value as usize)),
            _ => Err(SynacorError::InvalidArgument(value)),
        }
    }
}
