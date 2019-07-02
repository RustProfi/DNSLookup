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
mod response;
use response::parse_response;
mod question;
use customerror::CustomError;
use std::process::exit;
use crate::question::{Header, Question, Qtype, sock_send};


fn main() {
    let args: Vec<String> = env::args().collect();
    //let message = b"\xAA\xAA\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x07\x65\x78\x61\x6d\x70\x6c\x65\x03\x63\x6f\x6d\x00\x00\x01\x00\x01";
    if args.len() == 2 && &args[1] != "-help" {
        if check_ip(&args[1]) {
            let header = match Header::new(1,false,true) {
                Ok(s) => s,
                Err(e) => {
                    println!("{}", e.to_string());
                    exit(1)
                }
            };

            let message = Question::new(header.clone(), "", Qtype::A);
            sock_send(message);
            let message = Question::new(header, "", Qtype::AAAA);
            sock_send(message);
        } else {
            let header = match Header::new(2,false,false) {
                Ok(x) => x,
                Err(e) => {
                    println!("{}", e.to_string());
                    exit(1)
                }
            };
            let message = Question::new(header.clone(), &args[1], Qtype::A);
            sock_send(message);
            let message = Question::new(header, &args[1], Qtype::AAAA);
            sock_send(message)
        }
    } else {
        println!("Usage is: nslookup [Host Name] | [Host IP] | -help");
        println!("nslookup foo.bar.com (Returns IP Address for Host Name)");
        println!("nslookup 123.123.123.123 (Returns Host Name for Address)");
        println!("nslookup -help (Returns this Help Message)");
    }
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
