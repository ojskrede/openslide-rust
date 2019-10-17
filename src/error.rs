/// Errors that can occur in this library.
///
/// This error handling is highly inspired by the error handling in the csv crate
///
/// (https://docs.rs/csv/1.1.1/csv/struct.Error.html)[https://docs.rs/csv/1.1.1/csv/struct.Error.html]
///

use std::{fmt, error, ffi};

/// The specific error type
#[derive(Debug)]
pub enum ErrorKind {

    /// Error from the original library.
    ///
    /// Whenever operations are done on the underlying openslide object, its error status is
    /// checked. If it is non-null, the underlying openslide object is closed, and the message is
    /// reported via this error kind.
    NonNullErrorState {
        from_function: String,
        message: String,
    },

    /// Catch-all error from the original C library not covered by ErrorKind::NonNullErrorState
    Original {
        from_function: String,
        message: String,
    },

    /// Catches std::ffi::NulError from calling std::ffi::CString::new()
    Nul(ffi::NulError),

    /// Make sure clients do not rely on exhaustive matching as this library can add other error
    /// kinds in the future.
    #[doc(hidden)]
    __NonExhaustive,
}

impl ErrorKind {
    /// Gives the name of the function in the original implementation if the error originates
    /// there.
    pub fn from_function(&self) -> Option<String> {
        match *self {
            ErrorKind::NonNullErrorState { ref from_function, .. } => Some(from_function.clone()),
            ErrorKind::Original { ref from_function, .. } => Some(from_function.clone()),
            _ => None
        }
    }
    
    /// Gives the error message from the error in the original implementation if the error
    /// originates there.
    pub fn message(&self) -> Option<String> {
        match *self {
            ErrorKind::NonNullErrorState { from_function: _, ref message } => Some(message.clone()),
            ErrorKind::Original { from_function: _, ref message } => Some(message.clone()),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: Box<ErrorKind>,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Error { kind: Box::new(kind) }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn into_kind(self) -> ErrorKind {
        *self.kind
    }

    pub fn from_function(&self) -> Option<String> {
        self.kind.from_function()
    }

    pub fn message(&self) -> Option<String> {
        self.kind.message()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self.kind {
            ErrorKind::NonNullErrorState { ref from_function, ref message } => {
                write!(
                    f,
                    "ERROR: The slide object was in a non-null error state and has been closed.\n\
                    In function {} from the original C library: {}",
                    from_function.clone(),
                    message.clone(),
                )
            },
            ErrorKind::Original { ref from_function, ref message } => {
                write!(
                    f,
                    "ERROR: General error in function {} from the original C library: {}",
                    from_function.clone(),
                    message.clone(),
                )
            },
            ErrorKind::Nul(ref err) => err.fmt(f),
            _ => unreachable!(),
        }
    }
}

impl From<ffi::NulError> for Error {
    fn from(err: ffi::NulError) -> Error {
        Error::new(ErrorKind::Nul(err))
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self.kind {
            ErrorKind::NonNullErrorState { .. } => None,
            ErrorKind::Original { .. } => None,
            ErrorKind::Nul(ref err) => Some(err),
            _ => unreachable!(),
        }
    }
}
