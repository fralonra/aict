use std::{error, fmt};

use crate::aictable::Aictable;

/// Error types.
#[derive(Debug)]
pub enum Error<T: Aictable> {
    /// This error occurs when an ID already exists in the [`Factory`](crate::Factory).
    AlreadyExist(T),
    /// This error occurs when the maximum value for the type has been reached.
    MaxReached,
    /// This error occurs when there are no more IDs left to generate.
    OutOfSpace,
}

impl<T: Aictable> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AlreadyExist(id) => write!(f, "Id {:?} already exists", id),
            Self::MaxReached => write!(f, "Maximum reached"),
            Self::OutOfSpace => write!(f, "No more id left"),
        }
    }
}

impl<T: Aictable> error::Error for Error<T> {}
