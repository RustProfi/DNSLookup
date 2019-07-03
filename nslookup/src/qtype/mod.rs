use crate::customerror::CustomError;
use std::fmt;

pub enum Qtype {
    A,
    AAAA,
    CNAME
}

impl fmt::Display for Qtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Qtype::A => write!(f, "A"),
            Qtype::AAAA => write!(f, "AAAA"),
            Qtype::CNAME => write!(f, "CNAME"),
        }
    }
}

impl Qtype {
    pub fn value(&self) -> u8 {
        match *self {
            Qtype::A => 1 as u8,
            Qtype::AAAA => 28 as u8,
            Qtype::CNAME => 5 as u8,
        }
    }

    pub fn get_qtype(value: usize) -> Result<Self, CustomError> {
        match value {
            1 => Ok(Qtype::A),
            28 => Ok(Qtype::AAAA),
            5_ => Ok(Qtype::CNAME),
            _ => Err(CustomError::QtypeNotSupported(value))
        }
    }
}