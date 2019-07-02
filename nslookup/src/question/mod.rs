use crate::customerror::CustomError;
use crate::decode;
use std::env;
use std::net::{UdpSocket};
use std::io::Error;
use std::str::Utf8Error;
use std::str;
use std::num::ParseIntError;
use std::fmt::Write;
use std::num;
use std::fmt;
use std::vec::Vec;

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

impl Question {
    pub fn new(header: Vec<u8>, url: &str, qtype: bool) -> Vec<u8> {
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
            let rest = vec![0,0,qtype as u8,0,1];
            vec.extend(rest);
            vec
        } else {
            let rest = vec![0,0,qtype as u8,0,1];
            vec.extend(rest);
            vec
        }
    }
}

impl Header {
    pub fn new(id: u16, qr: bool, opcode: bool) -> Result<Vec<u8>,CustomError> {
        let queryparams = format!("{}000{}00100000000", qr as i32, opcode as i32);
        let m = format!("{:0>4x}{}0001000000000000",id, binary_to_hex(queryparams)?);
        Ok(decode(&m).unwrap())
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
        write!(&mut s, "{:x}", recursive_find(0, &x))?;
    }
    Ok(s)
}

/// returns u16 representation of the given chunk so that it can be formatet and concatenated to a hex
/// # Arguments
/// * `number`
fn recursive_find(mut number: u16, chars: &[char]) -> u16 {
    let mut chars_rec = chars.to_owned();
    let position = chars_rec.iter().position(|&x| x == '1');
    if position.is_some() {
        chars_rec[position.unwrap()] = '0';
        number += recursive_find(u16_from_position(position.unwrap()), &chars_rec);

    }
    number
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