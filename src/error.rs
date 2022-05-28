use std::{
    convert::From,
    error::Error,
    fmt::{self, Display},
    io,
    num::ParseFloatError,
    process,
};

#[derive(Debug)]
pub enum MatrixError {
    Io(io::Error),
    Parse(ParseFloatError),
    NotATriplet,
}

impl Error for MatrixError {}

impl Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatrixError::Io(e) => write!(f, "{}", e),
            MatrixError::Parse(e) => write!(f, "{}", e),
            MatrixError::NotATriplet => write!(f, "line is not a triplet"),
        }
    }
}

impl From<io::Error> for MatrixError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ParseFloatError> for MatrixError {
    fn from(x: ParseFloatError) -> Self {
        Self::Parse(x)
    }
}

/// Print display-formatted error if a result is an error.
/// Otherwise unwrap the value.
pub trait UnwrapOrDisplay<T, E> {
    fn unwrap_or_display(self) -> T;
}

impl<T, E> UnwrapOrDisplay<T, E> for Result<T, E>
where
    E: fmt::Display,
{
    fn unwrap_or_display(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: {}\n\nSee --help", e);
                process::exit(1);
            }
        }
    }
}
