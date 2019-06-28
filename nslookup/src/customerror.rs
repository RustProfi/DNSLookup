use std::fmt;

pub enum CustomError {
    IoError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
    ResponseError
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CustomError::IoError(ref e) => write!(f, "{}", e),
            CustomError::Utf8Error(ref e) => write!(f, "{}", e),
            CustomError::ResponseError => write!(f, "Response ist fehlerbehaftet.")
        }
    }
}

impl fmt::Debug for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CustomError::IoError(ref e) => write!(f, "{}", e),
            CustomError::Utf8Error(ref e) => write!(f, "{}", e),
            CustomError::ResponseError => write!(f, "Response ist fehlerbehaftet.")
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