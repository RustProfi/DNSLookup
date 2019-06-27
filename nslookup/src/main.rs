use std::env;
use std::net::{ToSocketAddrs, SocketAddr, UdpSocket};
use std::vec::IntoIter;
use std::io::Error;
use std::str;
use std::num::ParseIntError;
use std::fmt::Write;
use std::num;

struct Header {
    /// Header for our query
    id: u16,
    qr: bool,
    opcode: bool,
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
    if args.len() == 2 && &args[1] != "-help" {
        if check_ip(&args[1]) {
            println!("reverse lookup")
        } else {
            let mut buf = [0u8;4096];
            sock.send_to(&message[..],"141.37.11.1:53");
            let (amt, src) = sock.recv_from(&mut buf).unwrap();
            println!("{:?}", Vec::from(&buf[0..amt]));
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
