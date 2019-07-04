use crate::customerror::CustomError;
use std::fmt;

/// Variation of Qtype
#[derive(PartialEq)]
pub enum Qtype {
    /// IPv4
    A,
    /// IPv6
    AAAA,
    /// canonical name record
    CNAME
}

impl fmt::Display for Qtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Qtype::A => write!(f, "IPv4"),
            Qtype::AAAA => write!(f, "IPv6"),
            Qtype::CNAME => write!(f, "CNAME"),
        }
    }
}

impl fmt::Debug for Qtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Qtype::A => write!(f, "IPv4"),
            Qtype::AAAA => write!(f, "IPv6"),
            Qtype::CNAME => write!(f, "CNAME"),
        }
    }
}

impl Qtype {
    /// Returns the the Enum value
    pub fn value(&self) -> u8 {
        match *self {
            Qtype::A => 1 as u8,
            Qtype::AAAA => 28 as u8,
            Qtype::CNAME => 5 as u8,
        }
    }
    /// Returns Enum variant for given value
    ///
    /// # Arguments
    /// `value` - actual Qtype value
    pub fn get_qtype(value: usize) -> Result<Self, CustomError> {
        match value {
            1 => Ok(Qtype::A),
            28 => Ok(Qtype::AAAA),
            5 => Ok(Qtype::CNAME),
            _ => Err(CustomError::QtypeNotSupported(value))
        }
    }
}