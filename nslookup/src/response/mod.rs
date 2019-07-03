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
use crate::qtype::Qtype;
use std::intrinsics::transmute;


pub struct Response {
    pub name: String,
    pub ip: Vec<Ip>
}

pub struct Ip {
    pub ip: String,
    pub qtype: Qtype,
}

impl Ip {
    fn new(ip: String, qtype: Qtype) -> Self {
        Ip{ip: ip, qtype: qtype}
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut res_str = String::new();

        for xd in &self.ip/*[0..&self.ip.len()-1]*/ {
            res_str.push_str(&xd.qtype.to_string());
            res_str.push_str(" ");
            res_str.push_str(&xd.ip);
            res_str.push_str("\n");
        }
        //res_str.push_str(&self.ip[&self.ip.len() -1]);
        write!(f, "Domain: {}\nIps:\n{}", self.name, res_str)
    }
}

impl Response {
    fn new(name: String, ip: Vec<Ip>) -> Self {
        Response{name: name, ip: ip}
    }
}

pub fn parse_response(buf: &[u8], response_start_index: usize)-> Result<Response, CustomError> {
    //Todo: Unwrap iwie raus xD

    check_response_status(&to_binary_vec(&buf[2..4])?)?;
    let name = ((buf[response_start_index] as u16) << 8) | buf[response_start_index + 1] as u16;
    let domain_name_index = get_name_index(name);

    Ok(Response::new(getDomainR(&buf, domain_name_index), getIpR(&buf, response_start_index)?))
}



//Todo: ipv6 noch überprüfen
fn getIpR(response: &[u8], response_start_index: usize) -> Result<Vec<Ip>, CustomError> {

    let mut res = vec![];

    let qtype = Qtype::getQtype((((response[response_start_index+2] as u16) << 8) | response[response_start_index + 3] as u16) as usize)?;
    let length = (((response[response_start_index+10] as u16) << 8) | response[response_start_index + 11] as u16) as usize;
    let ip_start_index = response_start_index + 12;

    match qtype {
        Qtype::A => {
            let mut ip = (ip_start_index..ip_start_index+length).map(|a|format!("{}.", response[a])).collect::<String>();
            ip.pop();
            res.push( Ip::new(ip, qtype))
        },
        Qtype::AAAA => {
            //Todo: Richtige Darstellung
            let encoded = encode(&response[ip_start_index..ip_start_index+length]).chars().collect::<Vec<char>>();
            res.push( Ip::new(format_ipv6(&encoded), qtype))
        },
        Qtype::CNAME => {
            println!("clen is {}", length);
            let ip = (ip_start_index..ip_start_index+length).map(|a|response[a] as char).collect::<String>();
            res.push(Ip::new(ip, qtype))
        }
    }

    ;
    let next = ip_start_index+length;
    if next >= response.len() {
        Ok(res)
    }
    else {
        res.append( &mut getIpR(response, next)?);
        Ok(res)
    }
}

fn format_ipv6(chars: &[char]) -> String {
    //Nach RFC 5952
    let mut res = String::new();
    let mut doubleset = false;
    let mut doubleoff = false;

    for c in chars.chunks(4) {
        match c.iter().position(|&x| x != '0') {
            Some(v) => {
                res.push_str(&c[v..].iter().collect::<String>());
                res.push_str(":");
                if(doubleset) {
                    doubleoff = true;
                }

            },
            None => {
                if !doubleset {
                    res.push_str(":");
                    doubleset = true
                }
                else if doubleoff {
                    res.push_str("0:")
                }
            }
        }
    }
    res.pop();
    res
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

fn getDomainR(response: &[u8], index: usize) -> String {
    let length = response[index] as usize;
    let startindex = index + 1;
    let next = startindex+length;
    let domain_part = (startindex..startindex+length).map(|a|response[a] as char).collect::<String>();
    if response[next] == 0 {
        domain_part
    }
    else {
        format!("{}.{}", domain_part, getDomainR(response,next))
    }
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
