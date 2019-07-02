use std::fmt;

pub enum CustomError {
    Overflow,
    ParseIntError(std::num::ParseIntError),
    FmtError(std::fmt::Error),
    IoError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
    ResponseError,
    FaultyHexError,
    Message(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CustomError::ParseIntError(ref e) => write!(f, "{}", e),
            CustomError::FmtError(ref e) => write!(f, "{}", e),
            CustomError::IoError(ref e) => write!(f, "{}", e),
            CustomError::Utf8Error(ref e) => write!(f, "{}", e),
            CustomError::Overflow => write!(f, "More than usize:Max nonmatching elements"),
            CustomError::ResponseError => write!(f, "Response ist fehlerbehaftet."),
            CustomError::FaultyHexError => write!(f, "Given value was not hex."),
            CustomError::Message(ref e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Debug for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CustomError::ParseIntError(ref e) => write!(f, "{}", e),
            CustomError::FmtError(ref e) => write!(f, "{}", e),
            CustomError::IoError(ref e) => write!(f, "{}", e),
            CustomError::Utf8Error(ref e) => write!(f, "{}", e),
            CustomError::Overflow => write!(f, "More than usize:Max nonmatching elements"),
            CustomError::ResponseError => write!(f, "Response ist fehlerbehaftet."),
            CustomError::FaultyHexError => write!(f, "Given value was not hex."),
            CustomError::Message(ref e) => write!(f, "{}", e),
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

impl From<std::num::ParseIntError> for CustomError {
    fn from(error: std::num::ParseIntError) -> Self {
        CustomError::ParseIntError(error)
    }
}