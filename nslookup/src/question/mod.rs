use crate::customerror::CustomError;
use std::env;
use std::net::{UdpSocket};
use std::io::Error;
use std::str::Utf8Error;
use std::str;

use std::fmt::Write;
use std::num;
use std::fmt;
use std::vec::Vec;
use std::num::ParseIntError;

pub struct Header {
    /// Header for our query
    pub id: u16,
    pub qr: bool,
    pub opcode: bool,
}

pub struct Question {
    /// Question that gets send
    pub header: Vec<u8>,
    pub url: String,
    pub qtype: bool,
}

pub enum Qtype {
    A,
    AAAA
}

impl Qtype {
    fn value(&self) -> u8 {
        match *self {
            Qtype::A => 1 as u8,
            Qtype::AAAA => 28 as u8
        }
    }
}

impl Question {
    pub fn new(header: Vec<u8>, url: &str, qtype: Qtype) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.extend(header);
        if url.len() > 0 {
            for x in url.split(".").collect::<Vec<&str>>() {
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
    pub fn new(id: u16, qr: bool, opcode: bool) -> Result<Vec<u8>,CustomError> {
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

/// returns u16 representation of the given chunk so that it can be formatet and concatenated to a hex
/// # Arguments
/// * `number`
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
