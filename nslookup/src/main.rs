use std::env;
use std::str;
use std::vec::Vec;

mod customerror;
mod response;
mod question;
mod qtype;
use std::process::exit;
use crate::qtype::Qtype;
use crate::question::{Header, Question};
use std::net::UdpSocket;
use crate::response::parse_response;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && &args[1] != "-help" {
        if check_ip(&args[1]) {
            println!("Reverse lookup not supported");
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
        println!("Usage is: nslookup [Host Name] | -help");
        //println!("Usage is: nslookup [Host Name] | [Host IP] | -help");
        println!("nslookup foo.bar.com (Returns IP Address for Host Name)");
        //println!("nslookup 123.123.123.123 (Returns Host Name for Address)");
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

/// prints Records of the receiving packet
/// # Arguments
/// * `message` - u8 vector, containing Header and Question
pub fn sock_send(message: Vec<u8>) {
    let sock = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(e) =>
            {
                println!("{}", e.to_string());
                exit(1)
            }
    };
    let mut buf = [0u8;4096];
    match sock.send_to(&message[..],"1.1.1.1:53") {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e.to_string());
            exit(1)
        }
    }
    let amt = match sock.recv(&mut buf) {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e.to_string());
            exit(1)
        }
    };
    match parse_response(&buf[0..amt], message[..].len()) {
        Ok(response) => println!("{}",response.to_string()),
        Err(e) => println!("{}", e.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_ip_url() {
        assert_eq!(check_ip("127.0.0.1"), true);
    }
    #[test]
    fn test_check_ip_ip() {
        assert_eq!(check_ip("google.com"), false);
    }
    #[test]
    fn test_check_ip_nonip() {
        assert_eq!(check_ip("127.0.0.1.1"), false);
    }
}
