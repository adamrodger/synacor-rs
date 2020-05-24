use std::{error::Error, fmt::Display, io};

/// Custom error type
#[derive(Debug)]
pub enum SynacorError {
    InvalidArgument(u16),
    ParseError(std::io::Error),
    UnsupportedOpCode(u16),
}

impl Display for SynacorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SynacorError::InvalidArgument(n) => write!(f, "Invalid pointer address {}", n),
            SynacorError::ParseError(ref e) => e.fmt(f),
            SynacorError::UnsupportedOpCode(n) => {
                write!(f, "Unsupported opcode encountered: {}", n)
            }
        }
    }
}

impl Error for SynacorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SynacorError::InvalidArgument(_) => None,
            SynacorError::ParseError(ref e) => Some(e),
            SynacorError::UnsupportedOpCode(_) => None,
        }
    }
}

impl From<io::Error> for SynacorError {
    fn from(error: io::Error) -> Self {
        SynacorError::ParseError(error)
    }
}
