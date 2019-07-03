use crate::customerror::CustomError;
use crate::qtype::Qtype;
use std::str;
use std::fmt::Write;
use std::vec::Vec;
use std::num::ParseIntError;

/// A DNS Header is represented here
pub struct Header {
    pub id: u16,
    pub qr: bool,
    pub opcode: bool,
}

/// A DNS Question is represented here
pub struct Question {
    pub header: Vec<u8>,
    pub url: String,
    pub qtype: Qtype,
}

impl Question {
    /// Returns a u8 Vec made up of header and question
    ///
    /// # Arguments
    ///
    /// * `header` - u8 Vec of specified header
    /// * `url` - url
    /// * `qtype` - qtype (A or AAAA)
    pub fn new_question(header: Vec<u8>, url: &str, qtype: Qtype) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.extend(header);
        if !url.is_empty() {
            for x in url.split('.').collect::<Vec<&str>>() {
                let url_bytes: Vec<_> = x.bytes().collect();
                let len = url_bytes.len().to_be_bytes().to_vec();
                let mut question: Vec<_> = len.into_iter().filter(|&i| i != 0).collect();
                question.extend(url_bytes);
                vec.extend(question)

            }
            let rest = vec![0,0,qtype.value(),0,1];
            vec.extend(rest);
            vec
        } else {
            let rest = vec![0,0,qtype.value(),0,1];
            vec.extend(rest);
            vec
        }
    }
}

impl Header {
    /// Returns the header as u8 Vector
   ///
   /// # Arguments
   ///
   /// * `id` - arbitrary 16 bit identifier
   /// * `qr` - specify if query or response
   /// * `opcode` - qtype (A or AAAA) query type (standard/inverse)
    pub fn new_message(id: u16, qr: bool, opcode: bool) -> Result<Vec<u8>,CustomError> {
        let queryparams = format!("{}000{}00100000000", qr as i32, opcode as i32);
        let m = format!("{:0>4x}{}0001000000000000",id, binary_to_hex(queryparams)?);
        Ok(decode(&m)?)
    }
}

/// returns binary as hex representation
/// # Arguments
/// * `binary` 16 bit binary
fn binary_to_hex(binary: String) -> Result<String, CustomError> {
    let mut s = String::with_capacity(4);
    let i = binary.chars().collect::<Vec<char>>();
    let slice = i.chunks_exact(4);
    for x in slice {
        write!(&mut s, "{:x}", recursive_find(0, &x)?)?;
    }
    Ok(s)
}

/// returns u16 representation of the given chunk so that it can be formated and concatenated to a hex
/// # Arguments
/// * `number` - number
/// * `chars` - chunk
fn recursive_find(mut number: u16, chars: &[char]) -> Result<u16, CustomError> {
    let mut chars_rec = chars.to_owned();
    let position = chars_rec.iter().position(|&x| x == '1');
    if position.is_some() {
        let pos = match position {
            Some(e) => e,
            None => return Err(CustomError::Overflow)
        };
        chars_rec[pos] = '0';
        number += recursive_find(u16_from_position(pos), &chars_rec)?;

    }
    Ok(number)
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
        assert_eq!(binary_to_hex(bin.to_string()).unwrap(), "0900");
    }
}