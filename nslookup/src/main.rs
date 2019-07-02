use std::env;
use std::net::{UdpSocket};
use std::io::Error;
use std::str::Utf8Error;
use std::str;
use std::num::ParseIntError;
use std::fmt::Write;
use std::vec::Vec;

mod customerror;
use customerror::CustomError;
use std::process::exit;

struct Header {
    /// Header for our query
    id: u16,
    qr: bool,
    opcode: bool,
}

enum Ip {
    IpV4(String), IpV6(String)
}

struct Response {
    name: String,
    ip: Ip
}

struct Question {
    /// Question that gets send
    header: Vec<u8>,
    url: String,
    qtype: bool,
}

impl Question {
    fn new(header: Vec<u8>, url: &str, qtype: bool) -> Vec<u8> {
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
    fn new(id: u16, qr: bool, opcode: bool) -> Result<Vec<u8>,CustomError> {
        let queryparams = format!("{}000{}00100000000", qr as i32, opcode as i32);
        let m = format!("{:0>4x}{}0001000000000000",id, binary_to_hex(queryparams)?);
        Ok(decode(&m).unwrap())
    }
}

fn main() {
    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && &args[1] != "-help" {
        if check_ip(&args[1]) {
            let header = match Header::new(43691,false,true) {
                Ok(x) => x,
                Err(e) => {
                    println!("{}", e.to_string());
                    exit(1)
                }
            };
            let message = Question::new(header, "", true);
            let mut buf = [0u8;4096];
            sock.send_to(&message[..],"1.1.1.1:53").unwrap();
            sock.recv(&mut buf).unwrap();
        } else {
            let header = match Header::new(43690,false,false) {
                Ok(x) => x,
                Err(e) => {
                    println!("{}", e.to_string());
                    exit(1)
                }
            };
            let message = Question::new(header, &args[1], true);
            let mut buf = [0u8;4096];
            sock.send_to(&message[..],"1.1.1.1:53").unwrap();
            let amt = sock.recv(&mut buf).unwrap();
            //println!("{:x?}", Vec::from(&buf[0..amt]));
            match parse_response(&buf[0..amt]) {
                Ok(xD) => println!("{:?}", xD),
                Err(lol) => println!("{:#?}", lol.to_string())
            }
        }
    } else {
        println!("Usage is: nslookup [Host Name] | [Host IP] | -help");
        println!("nslookup foo.bar.com (Returns IP Address for Host Name)");
        println!("nslookup 123.123.123.123 (Returns Host Name for Address)");
        println!("nslookup -help (Returns this Help Message)");
    }
}

fn parse_response(buf: &[u8])-> Result<String, CustomError> {
    let asHex = encode(buf);
    let vec = asHex.as_bytes().chunks(2).map(str::from_utf8).collect::<Result<Vec<&str>, _>>()?;
    println!("{:?}", vec);
    Ok(asHex)
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
        write!(&mut s, "{:02x}", b).unwrap();
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