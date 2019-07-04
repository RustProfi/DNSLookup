extern crate nslookup;
use std::env;
use std::str;
use std::vec::Vec;

mod customerror;
mod response;
mod question;
mod qtype;
use std::process::exit;
use crate::qtype::Qtype;
use std::net::UdpSocket;
use crate::response::Response;
use crate::question::{Header, Question, DnsMessageBuilder};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && &args[1] != "-help" {
        if check_ip(&args[1]) {
            println!("Reverse lookup not supported");
        } else {
            let header = Header::new(2, false, false);
            let mut messages = vec![];
            messages.push(Question::new(&args[1], Qtype::A));
            messages.push(Question::new(&args[1], Qtype::AAAA));

            let pack = match DnsMessageBuilder::new(header, messages).build_messages() {
                Ok(val) => val,
                Err(e) => {
                    println!("{}", e);
                    exit(1)
                }
            };
            sock_send(pack)
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
    ip.split('.').map(|s| s.parse::<i32>().is_ok()).count() == 4
}

/// Creates a DNS-Package which contains a Header and a Question.
/// Sends and retreives it.
/// Prints Records of the receiving packet.
/// The response contains the same Header + Question and in addition one or more concatenated Answers.
/// # Arguments
/// * `message` - u8 vector, containing Header and Question
pub fn sock_send(messages: Vec<Vec<u8>>) {
    for message in messages {
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
        match Response::parse_response(&buf[0..amt], message[..].len()) {
            Ok(response) => println!("{}",response.to_string()),
            Err(e) => println!("{}", e.to_string())
        }
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
