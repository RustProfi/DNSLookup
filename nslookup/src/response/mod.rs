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
use crate::customerror::CustomError;


pub struct Response {
    pub name: String,
    pub ip: Ip
}

pub enum Ip {
    IpV4(String), IpV6(String)
}

impl Response {
    fn new(name: String, ip: Ip) -> Self {
        Response{name: name, ip: ip}
    }
}

impl fmt::Display for Ip {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Ip::IpV4(ref ip) => write!(f, "{}", ip),
            Ip::IpV6(ref ip) => write!(f, "{}", ip),
        }
    }
}

pub fn parse_response(buf: &[u8], response_start_index: usize)-> Result<Response, CustomError> {
    //Todo: Unwrap iwie raus xD
    check_response_status(&to_binary_vec(&buf[2..4])?)?;

    let domain_name_index = get_name_index(((buf[response_start_index] as u16) << 8) | buf[response_start_index + 1] as u16);
    Ok(Response::new(getDomain(&buf, domain_name_index)?, getIp(&buf, response_start_index, true)?))
}

//Todo: ipv6 noch überprüfen
fn getIp(response: &[u8], response_start_index: usize, ipv4: bool) -> Result<Ip, CustomError> {
    let length = (((response[response_start_index+10] as u16) << 8) | response[response_start_index + 11] as u16) as usize;
    let ip_start_index = response_start_index + 12;

    let ipv4 = (ip_start_index..ip_start_index+length).map(|a|format!("{}.", response[a])).collect::<String>().trim_end_matches(".").to_string();
    Ok((Ip::IpV4(ipv4)))
}

fn get_name_index(bytes: u16) -> usize {
    (bytes << 2 >> 2) as usize
}

fn to_binary_vec(buf: &[u8]) -> Result<Vec<u8>, CustomError> {
    let bytes = encode(&buf);
    let mut s =  String::with_capacity(bytes.len() * 4);

    for i in bytes.chars() {
        match i {
            '0' => write!(&mut s, "0000"),
            '1' => write!(&mut s, "0001"),
            '2' => write!(&mut s, "0010"),
            '3' => write!(&mut s, "0011"),
            '4' => write!(&mut s, "0100"),
            '5' => write!(&mut s, "0101"),
            '6' => write!(&mut s, "0110"),
            '7' => write!(&mut s, "0111"),
            '8' => write!(&mut s, "1000"),
            '9' => write!(&mut s, "1001"),
            'a' | 'A' => write!(&mut s, "1010"),
            'b' | 'B' => write!(&mut s, "1011"),
            'c' | 'C' => write!(&mut s, "1100"),
            'd' | 'D' => write!(&mut s, "1101"),
            'e' | 'E' => write!(&mut s, "1110"),
            'f' | 'F' => write!(&mut s, "1111"),
            _ => return Err(CustomError::FaultyHexError),
        };
    }
    Ok(s.chars().map(|a| a.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
}


//Todo: index out of bounds abfangen
fn getDomain(response: &[u8], answerstart: usize) -> Result<String, CustomError> {
    let length1 = response[answerstart] as usize;
    let startindex = answerstart + 1;
    let length2 = response[startindex + length1] as usize;
    let startindex2 = startindex + length1 + 1;

    let mut s = String::with_capacity(length1 + length2 + 1);
    //println!("{}", length2);
    //println!("{}", response[startindex+1] as char);

    for c in startindex..startindex+length1 {
        write!(&mut s, "{}", response[c] as char);
    }
    write!(&mut s, "{}", '.');

    for c in startindex2..startindex2+length2 {
        write!(&mut s, "{}", response[c] as char);
    }

    Ok(s)
}

fn check_response_status(bytes: &[u8]) -> Result<(), CustomError> {
    //println!("{:?}", bytes);
    //bytes[0] ist response,
    if bytes.len() < 16 || bytes[0] == 0 || bytes[12] == 1 || bytes[13] == 1 || bytes[14] == 1 || bytes[15] == 1 {
        Err(CustomError::ResponseError)
    }
    else {
        Ok(())
    }
}

/// Returns hex of a u8
/// # Arguments
/// * `bytes` - bytes that we want to parse
fn encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}
