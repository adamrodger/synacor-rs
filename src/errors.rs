use std::{error::Error, fmt::Display, io};

/// Custom error type
#[derive(Debug)]
pub enum SynacorError {
    EmptyStack,
    InvalidArgument(u16),
    ParseError(std::io::Error),
    ReadOnly(u16),
    UnsupportedOpCode(u16),
}

impl Display for SynacorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SynacorError::EmptyStack => write!(f, "Attempt to read from empty stack"),
            SynacorError::InvalidArgument(n) => write!(f, "Invalid pointer address {}", n),
            SynacorError::ParseError(ref e) => e.fmt(f),
            SynacorError::ReadOnly(n) => write!(
                f,
                "Attempted to write to read-only memory at position {}",
                n
            ),
            SynacorError::UnsupportedOpCode(n) => {
                write!(f, "Unsupported opcode encountered: {}", n)
            }
        }
    }
}

impl Error for SynacorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SynacorError::EmptyStack => None,
            SynacorError::InvalidArgument(_) => None,
            SynacorError::ParseError(ref e) => Some(e),
            SynacorError::ReadOnly(_) => None,
            SynacorError::UnsupportedOpCode(_) => None,
        }
    }
}

impl From<io::Error> for SynacorError {
    fn from(error: io::Error) -> Self {
        SynacorError::ParseError(error)
    }
}
