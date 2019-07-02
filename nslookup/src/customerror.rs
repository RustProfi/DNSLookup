use std::fmt;

pub enum CustomError {
    FmtError(std::fmt::Error),
    IoError(std::io::Error),
    Utf8Error(std::str::Utf8Error)
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CustomError::FmtError(ref e) => write!(f, "{}", e),
            CustomError::IoError(ref e) => write!(f, "{}", e),
            CustomError::Utf8Error(ref e) => write!(f, "{}", e)
        }
    }
}

impl fmt::Debug for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CustomError::FmtError(ref e) => write!(f, "{}", e),
            CustomError::IoError(ref e) => write!(f, "{}", e),
            CustomError::Utf8Error(ref e) => write!(f, "{}", e)
        }
    }
}

impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError::IoError(error)
    }
}

impl From<std::str::Utf8Error> for CustomError {
    fn from(error: std::str::Utf8Error) -> Self {
        CustomError::Utf8Error(error)
    }
}

impl From<std::fmt::Error> for CustomError {
    fn from(error: std::fmt::Error) -> Self {
        CustomError::FmtError(error)
    }
}