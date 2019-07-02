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

impl fmt::Display for Ip {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Ip::IpV4(ref ip) => write!(f, "{}", ip),
            Ip::IpV6(ref ip) => write!(f, "{}", ip),
        }
    }
}

impl Response {
    fn new(name: String, ip: Ip) -> Self {
        Response{name: name, ip: ip}
    }
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
    //let message = b"\xAA\xAA\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x07\x65\x78\x61\x6d\x70\x6c\x65\x03\x63\x6f\x6d\x00\x00\x01\x00\x01";
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
            match parse_response(&buf[0..amt], message[..].len()) {
                Ok(xD) => println!("{} {}",xD.name, xD.ip),
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

fn parse_response(buf: &[u8], response_start_index: usize)-> Result<Response, CustomError> {
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