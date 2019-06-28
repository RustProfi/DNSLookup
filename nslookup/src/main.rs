use std::env;
use std::net::{UdpSocket};
use std::str;
use std::num::ParseIntError;
use std::fmt::Write;
use std::vec::Vec;
use std::slice;
use std::path::Iter;

struct Header {
    /// Header for our query
    id: u16,
    qr: bool,
    opcode: bool,
}

impl Header {
    fn new(id: u16, qr: bool, opcode: bool) -> String {
        let bin = format!("{}000{}00100000000", qr, opcode);
        binary_to_hex(bin);
        format!("{:0>4x}",id, )
    }
}

fn main() {
    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    let message = b"\xAA\xAA\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x07\x65\x78\x61\x6d\x70\x6c\x65\x03\x63\x6f\x6d\x00\x00\x01\x00\x01";
    let s = encode(message);
    let bin = "1100100100000100".to_string();
    binary_to_hex(bin);
    //println!("{}", binary_to_u16(bin));
    println!("{}", s);
    println!("{:?}", decode(&s));
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && &args[1] != "-help" {
        if check_ip(&args[1]) {
            println!("reverse lookup")
        } else {
            let mut buf = [0u8;4096];
            sock.send_to(&message[..],"141.37.11.1:53");
            let amt = sock.recv(&mut buf).unwrap();
        }
    } else {
        println!("Usage is: nslookup [Host Name] | [Host IP] | -help");
        println!("nslookup foo.bar.com (Returns IP Address for Host Name)");
        println!("nslookup 123.123.123.123 (Returns Host Name for Address)");
        println!("nslookup -help (Returns this Help Message)");
    }
}

/// Returns a u8 vector on success else an Error
/// # Arguments
/// * `hex` - a hex &str that we want to parse
fn decode(hex: &str) -> Result<Vec<u8>,ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}

/// Returns hex of a u8
/// # Arguments
/// * `bytes` - bytes that we want to parse
fn encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b);
    }
    s
}

/// Returns true if argument is an IP
/// # Arguments
/// * `bytes` - bytes that we want to parse
fn check_ip(ip: &str) -> bool {
    if ip.split(".").count() == 4 {
        ip.split(".").all(|s| s.parse::<i32>().is_ok())
    } else {
        false
    }
}

fn binary_to_hex(binary: String) -> () {
    let mut s = String::with_capacity(4);
    let i = binary.chars().collect::<Vec<char>>();
    let mut slice = i.chunks_exact(4);
    for x in slice {
        write!(&mut s, "{:x}", recursive_find(0, &x));
    }
    println!("{}", s)
}
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