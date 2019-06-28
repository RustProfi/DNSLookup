use std::env;
use std::net::{ToSocketAddrs, SocketAddr, UdpSocket};
use std::vec::IntoIter;
use std::io::Error;
use std::str::Utf8Error;
use std::str;
use std::num::ParseIntError;
use std::fmt::Write;
use std::num;

mod customerror;
use customerror::CustomError;

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

impl Response {
    fn new(name: String, ip: Ip) -> Self {
        Response{name: name, ip: ip}
    }
}

impl Header {
    fn new(id: u16, qr: bool, opcode: bool) -> String {
        let bin = format!("{}000{}00100000000", qr, opcode);
        format!("{:0>4x}",id)
    }
}

fn main() {
    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    let args: Vec<String> = env::args().collect();
    let message = b"\xAA\xAA\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x07\x65\x78\x61\x6d\x70\x6c\x65\x03\x63\x6f\x6d\x00\x00\x01\x00\x01";
    if args.len() == 2 && &args[1] != "-help" {
        if check_ip(&args[1]) {
            println!("reverse lookup")
        } else {
            let mut buf = [0u8;4096];
            sock.send_to(&message[..],"141.37.11.1:53");
            let (amt, src) = sock.recv_from(&mut buf).unwrap();
            //println!("{:x?}", Vec::from(&buf[0..amt]));
            match parse_response(&buf[0..amt]) {
                Ok(xD) => println!("Passt"),
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

fn parse_response(buf: &[u8])-> Result<Response, CustomError> {
    let asHex = encode(buf);
    let response_vec = asHex.as_bytes().chunks(2).map(str::from_utf8).collect::<Result<Vec<&str>, _>>()?;
    println!("{:?}", response_vec);
    //println!("{}",format!("{}{}", vec[2], vec[3]));
    check_response_status(&hex_to_binary(format!("{}{}", response_vec[2], response_vec[3])))?;
    Ok(Response::new(String::from("amk"), Ip::IpV4(String::from("xD"))))
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

fn hex_to_binary(hex: String) -> Vec<u8> {
    let mut result = vec![];
    for i in hex.chars() {
        match i {
            '0' => {result.push(0); result.push(0); result.push(0); result.push(0)},
            '1' => {result.push(0); result.push(0); result.push(0); result.push(1)},
            '2' => {result.push(0); result.push(0); result.push(1); result.push(0)},
            '3' => {result.push(0); result.push(0); result.push(1); result.push(1)},
            '4' => {result.push(0); result.push(1); result.push(0); result.push(0)},
            '5' => {result.push(0); result.push(1); result.push(0); result.push(1)},
            '6' => {result.push(0); result.push(1); result.push(1); result.push(0)},
            '7' => {result.push(0); result.push(1); result.push(1); result.push(1)},
            '8' => {result.push(1); result.push(0); result.push(0); result.push(0)},
            '9' => {result.push(1); result.push(0); result.push(0); result.push(1)},
            'a' | 'A' => {result.push(1); result.push(0); result.push(1); result.push(0)},
            'b' | 'B' => {result.push(1); result.push(0); result.push(1); result.push(1)},
            'c' | 'C' => {result.push(1); result.push(1); result.push(0); result.push(0)},
            'd' | 'D' => {result.push(1); result.push(1); result.push(0); result.push(1)},
            'e' | 'E' => {result.push(1); result.push(1); result.push(1); result.push(0)},
            'f' | 'F' => {result.push(1); result.push(1); result.push(1); result.push(1)},
            _ => ()
        }
    }
    result
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
