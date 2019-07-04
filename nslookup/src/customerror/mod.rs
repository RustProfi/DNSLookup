use std::fmt;

pub enum CustomError {
    Overflow,
    ParseIntError(std::num::ParseIntError),
    FmtError(std::fmt::Error),
    IoError(std::io::Error),
    ResponseError,
    QtypeNotSupported(usize),
    EmptyResponse,
    IpParseError,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CustomError::ParseIntError(ref e) => write!(f, "{}", e),
            CustomError::FmtError(ref e) => write!(f, "{}", e),
            CustomError::IoError(ref e) => write!(f, "{}", e),
            CustomError::Overflow => {
                write!(f, "Ip would overflow. Something with the response is wrong")
            }
            CustomError::ResponseError => write!(f, "Response ist faulty."),
            CustomError::QtypeNotSupported(ref x) => write!(f, "Qtype {} is not supported", x),
            CustomError::EmptyResponse => write!(f, "Response is empty"),
            CustomError::IpParseError => write!(f, "Invalid Ip Adress"),
        }
    }
}

impl fmt::Debug for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CustomError::ParseIntError(ref e) => write!(f, "{}", e),
            CustomError::FmtError(ref e) => write!(f, "{}", e),
            CustomError::IoError(ref e) => write!(f, "{}", e),
            CustomError::Overflow => {
                write!(f, "Ip would overflow. Something with the response is wrong")
            }
            CustomError::ResponseError => write!(f, "Response ist faulty."),
            CustomError::QtypeNotSupported(ref x) => write!(f, "Qtype {} is not supported", x),
            CustomError::EmptyResponse => write!(f, "Response is empty"),
            CustomError::IpParseError => write!(f, "Invalid Ip Adress"),
        }
    }
}

impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError::IoError(error)
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
