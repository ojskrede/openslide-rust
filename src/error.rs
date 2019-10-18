/// Errors that can occur in this library.
///
/// This error handling is highly inspired by the error handling in the csv crate
///
/// (https://docs.rs/csv/1.1.1/csv/struct.Error.html)[https://docs.rs/csv/1.1.1/csv/struct.Error.html]
///
use std::{error, ffi, fmt, io, str};

/// The specific error type
#[derive(Debug)]
pub enum ErrorKind {
    /// Error from the original library.
    ///
    /// Whenever operations are done on the underlying openslide object, its error status is
    /// checked. If it is non-null, the underlying openslide object is closed, and the message is
    /// reported via this error kind.
    NonNullErrorState {
        in_function: String,
        message: String,
    },

    /// Error from the original library.
    ///
    /// Functions that are expected to return a value will instead return an error value if
    /// something goes wrong. This error covers these cases
    ReturnValue {
        in_function: String,
        message: String,
    },

    /// Error when converting to and from number types
    NumPrimitiveCast { message: String },

    /// Errors for values that are out of bounds
    OutOfBounds { message: String },

    /// Catches std::ffi::NulError from calling std::ffi::CString::new()
    Nul(ffi::NulError),

    /// Errors regarding I/O, also including file not found for the input slide path.
    Io(io::Error),

    /// Errors regarding utf8 strings
    Utf8(str::Utf8Error),

    /// Make sure clients do not rely on exhaustive matching as this library can add other error
    /// kinds in the future.
    #[doc(hidden)]
    __NonExhaustive,
}

impl ErrorKind {
    /// Gives the name of the function in the original implementation if the error originates
    /// there.
    pub fn in_function(&self) -> Option<String> {
        match *self {
            ErrorKind::NonNullErrorState {
                ref in_function, ..
            } => Some(in_function.clone()),
            ErrorKind::ReturnValue {
                ref in_function, ..
            } => Some(in_function.clone()),
            _ => None,
        }
    }

    /// Gives the error message from the error in the original implementation if the error
    /// originates there.
    pub fn message(&self) -> Option<String> {
        match *self {
            ErrorKind::NonNullErrorState { ref message, .. } => Some(message.clone()),
            ErrorKind::ReturnValue { ref message, .. } => Some(message.clone()),
            ErrorKind::NumPrimitiveCast { ref message } => Some(message.clone()),
            ErrorKind::OutOfBounds { ref message } => Some(message.clone()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: Box<ErrorKind>,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Error {
            kind: Box::new(kind),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn into_kind(self) -> ErrorKind {
        *self.kind
    }

    pub fn in_function(&self) -> Option<String> {
        self.kind.in_function()
    }

    pub fn message(&self) -> Option<String> {
        self.kind.message()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self.kind {
            ErrorKind::NonNullErrorState {
                ref in_function,
                ref message,
            } => write!(
                f,
                "ERROR: The slide object was in a non-null error state and has been closed.\n\
                 In function {} from the original C library: {}",
                in_function.clone(),
                message.clone(),
            ),
            ErrorKind::ReturnValue {
                ref in_function,
                ref message,
            } => write!(
                f,
                "ERROR: Returned error value in function {} from the original C library: {}",
                in_function.clone(),
                message.clone(),
            ),
            ErrorKind::NumPrimitiveCast { ref message } => write!(
                f,
                "ERROR: Converting between number types: {}",
                message.clone(),
            ),
            ErrorKind::OutOfBounds { ref message } => {
                write!(f, "ERROR: Value is out of bounds: {}", message.clone(),)
            }
            ErrorKind::Nul(ref err) => err.fmt(f), // TODO: If only used for CString::new(), specialise it
            ErrorKind::Io(ref err) => err.fmt(f),
            ErrorKind::Utf8(ref err) => err.fmt(f),
            _ => unreachable!(),
        }
    }
}

impl From<ffi::NulError> for Error {
    fn from(err: ffi::NulError) -> Error {
        Error::new(ErrorKind::Nul(err))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::new(ErrorKind::Io(err))
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::new(ErrorKind::Utf8(err))
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self.kind {
            ErrorKind::NonNullErrorState { .. } => None,
            ErrorKind::ReturnValue { .. } => None,
            ErrorKind::Nul(ref err) => Some(err),
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::Utf8(ref err) => Some(err),
            ErrorKind::NumPrimitiveCast { .. } => None,
            ErrorKind::OutOfBounds { .. } => None,
            _ => unreachable!(),
        }
    }
}
