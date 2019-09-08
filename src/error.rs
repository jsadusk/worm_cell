use std::error;
use std::fmt;

#[derive(Debug)]
pub enum WormCellError {
    ReadNotSet,
    DoubleSet
}

impl error::Error for WormCellError {}

impl fmt::Display for WormCellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadNotSet => write!(f, "Tried to read a WormReader that wasn't set"),
            Self::DoubleSet => write!(f, "Tried to set a WormCell twice")
        }
    }
}

pub type WormCellResult<T> = Result<T, WormCellError>;
