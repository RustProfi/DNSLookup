use crate::customerror::CustomError;
use crate::qtype::Qtype;
use std::fmt::Write;
use std::num::ParseIntError;
use std::str;
use std::vec::Vec;

/// Create DNS-Headers
pub struct Header {
    pub id: u16,
    pub qr: bool,
    pub opcode: bool,
}

impl Header {
    /// Returns a new Header
    ///
    /// # Arguments
    ///
    /// * `id` - arbitrary 16 bit identifier
    /// * `qr` - specify if query or response
    /// * `opcode` - qtype (A or AAAA) query type (standard/inverse)
    pub fn new(id: u16, qr: bool, opcode: bool) -> Self {
        Header { id, qr, opcode }
    }

    /// Parses the Header in an u8 Vector
    pub fn get_header(&self) -> Result<Vec<u8>, CustomError> {
        let queryparams = format!("{}000{}00100000000", self.qr as i32, self.opcode as i32);
        let m = format!(
            "{:0>4x}{}0001000000000000",
            self.id,
            binary_to_hex(&queryparams)?
        );
        Ok(decode(&m)?)
    }
}

/// Create DNS-Questions
pub struct Question {
    pub url: String,
    pub qtype: Qtype,
}

impl Question {
    /// Returns a new Question
    ///
    /// # Arguments
    ///
    /// * `header` - u8 Vec of specified header
    /// * `url` - url
    /// * `qtype` - qtype (A or AAAA)
    pub fn new(url: &str, qtype: Qtype) -> Self {
        Question {
            url: String::from(url),
            qtype,
        }
    }

    /// Parses the Question in an u8 Vector.
    pub fn get_question(&self) -> Vec<u8> {
        let mut vec = vec![];
        let rest = vec![0, 0, self.qtype.value(), 0, 1];
        if !self.url.is_empty() {
            for x in self.url.split('.').collect::<Vec<&str>>() {
                let url_bytes: Vec<_> = x.bytes().collect();
                let len = url_bytes
                    .len()
                    .to_be_bytes()
                    .to_vec()
                    .into_iter()
                    .filter(|&i| i != 0)
                    .collect::<Vec<_>>();
                vec.extend(len);
                vec.extend(url_bytes);
            }
            vec.extend(rest);
            vec
        } else {
            vec.extend(rest);
            vec
        }
    }
}

/// Use the DnsMessageBuilder to build DNS Messages
pub struct DnsMessageBuilder {
    pub header: Header,
    pub questions: Vec<Question>,
}

impl DnsMessageBuilder {
    /// Returns a new DNSPackage
    ///
    /// # Arguments
    ///
    /// * `header` - A header struct
    /// * `questions` - A vector of questions
    pub fn new(header: Header, questions: Vec<Question>) -> Self {
        DnsMessageBuilder { header, questions }
    }

    /// Builds the dns messages
    pub fn build_messages(&self) -> Result<Vec<Vec<u8>>, CustomError> {
        let mut res = vec![];
        for q in &self.questions {
            res.push([self.header.get_header()?, q.get_question()].concat());
        }
        Ok(res)
    }
}

/// returns binary as hex representation
/// # Arguments
/// * `binary` 16 bit binary
fn binary_to_hex(binary: &str) -> Result<String, CustomError> {
    let mut s = String::with_capacity(4);
    for x in binary.chars().collect::<Vec<char>>().chunks_exact(4) {
        write!(&mut s, "{:x}", recursive_find(0, &x))?;
    }
    Ok(s)
}

/// returns u16 representation of the given chunk so that it can be formated and concatenated to a hex
/// # Arguments
/// * `number` - number
/// * `chars` - chunk
fn recursive_find(number: u16, chars: &[char]) -> u16 {
    let mut result = number;
    let mut chars_rec = chars.to_owned();
    let position = chars_rec.iter().position(|&x| x == '1');

    match position {
        Some(val) => {
            chars_rec[val] = '0';
            result += recursive_find(u16_from_position(val), &chars_rec);
            result
        }
        None => result,
    }
}

/// returns u16 representation of given position
/// # Arguments
/// * `usize` position of 1
fn u16_from_position(position: usize) -> u16 {
    match position {
        0 => 8,
        1 => 4,
        2 => 2,
        3 => 1,
        _ => 0,
    }
}

/// Returns a u8 vector on success else an Error
/// # Arguments
/// * `hex` - a hex &str that we want to parse
fn decode(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_u16_from_position() {
        assert_eq!(u16_from_position(0), 8);
    }
    #[test]
    fn test_decode() {
        let message = "AAAA01000001000000000000076578616d706c6503636f6d0000010001";
        let message_decode = b"\xAA\xAA\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x07\x65\x78\x61\x6d\x70\x6c\x65\x03\x63\x6f\x6d\x00\x00\x01\x00\x01";
        assert_eq!(decode(message).unwrap(), message_decode);
    }
    #[test]
    fn test_binary_to_hex() {
        let bin = "0000100100000000";
        assert_eq!(binary_to_hex(bin).unwrap(), "0900");
    }
}
