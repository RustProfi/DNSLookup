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
            println!("{:x?}", &buf[0..amt]);
            match parse_response(&buf[0..amt]) {
                Ok(xD) => println!("{}",xD.name),
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
    //Todo: Unwrap iwie raus xD
    check_response_status(&to_binary_string(&buf[2..4])?.chars().map(|a| a.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())?;

    Ok(Response::new(getDomain(&buf)?, Ip::IpV4(String::from("xD"))))
}

fn to_binary_string(buf: &[u8]) -> Result<String, CustomError> {
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
    Ok(s)
}


//Todo: index out of bounds abfangen
fn getDomain(response: &[u8]) -> Result<String, CustomError> {
    let answerstart = 12;
    let length1 = response[answerstart] as usize;
    let startindex = answerstart + 1;
    let length2 = response[startindex + length1] as usize;
    let startindex2 = startindex + length1 + 1;

    let mut s = String::with_capacity(length1 + length2 + 1);
    println!("{}", length2);
    println!("{}", response[startindex+1] as char);

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
