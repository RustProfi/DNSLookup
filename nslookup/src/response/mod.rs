use std::fmt::Write;
use std::fmt;
use std::vec::Vec;
use crate::customerror::CustomError;
use crate::qtype::Qtype;

pub struct Response {
    pub name: String,
    pub ip: Vec<Ip>
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut res_str = String::new();
        for ip in &self.ip {
            res_str.push_str(&ip.qtype.to_string());
            res_str.push_str(": ");
            res_str.push_str(&ip.ip);
            res_str.push_str("\n");
        }
        write!(f, "Domain: {}\nAdress(es):\n{}", self.name, res_str)
    }
}

impl Response {
    fn new(name: String, ip: Vec<Ip>) -> Self {
        Response{name, ip}
    }
}

pub struct Ip {
    pub ip: String,
    pub qtype: Qtype,
}

impl Ip {
    fn new(ip: String, qtype: Qtype) -> Self {
        Ip{ip, qtype}
    }
}

pub fn parse_response(buf: &[u8], response_start_index: usize)-> Result<Response, CustomError> {
    if response_start_index >= buf.len() {
        return Err(CustomError::EmptyResponse);
    }
    check_response_status(&to_binary_vec(&buf[2..4])?)?;
    let name = ((u16::from(buf[response_start_index])) << 8) | u16::from(buf[response_start_index + 1]);
    let domain_name_index = get_name_index(name);

    Ok(Response::new(get_domain_r(&buf, domain_name_index), get_ip_r(&buf, response_start_index)?))
}

fn check_response_status(bytes: &[u8]) -> Result<(), CustomError> {
    if bytes.len() < 16 || bytes[0] == 0 || bytes[12] == 1 || bytes[13] == 1 || bytes[14] == 1 || bytes[15] == 1 {
        Err(CustomError::ResponseError)
    }
    else {
        Ok(())
    }
}

fn get_domain_r(response: &[u8], index: usize) -> String {
    let length = response[index] as usize;
    let startindex = index + 1;
    let next = startindex+length;
    let domain_part = (startindex..startindex+length).map(|a|response[a] as char).collect::<String>();
    if response[next] == 0 {
        domain_part
    }
    else {
        format!("{}.{}", domain_part, get_domain_r(response, next))
    }
}

fn get_ip_r(response: &[u8], response_start_index: usize) -> Result<Vec<Ip>, CustomError> {
    let mut result = vec![];

    let qtypehex = (((u16::from(response[response_start_index + 2])) << 8) | u16::from(response[response_start_index + 3])) as usize;
    let qtype = Qtype::get_qtype(qtypehex)?;

    let ip_length = (((u16::from(response[response_start_index + 10])) << 8) | u16::from(response[response_start_index + 11])) as usize;
    let ip_start_index = response_start_index + 12;

    match qtype {
        Qtype::A => {
            let mut ip = (ip_start_index..ip_start_index + ip_length).map(|a|format!("{}.", response[a])).collect::<String>();
            ip.pop();
            result.push( Ip::new(ip, qtype))
        },
        Qtype::AAAA => {
            let encoded = encode(&response[ip_start_index..ip_start_index+ip_length])?.chars().collect::<Vec<char>>();
            result.push( Ip::new(format_ipv6(&encoded), qtype))
        },
        Qtype::CNAME => {
            let ip = (ip_start_index..ip_start_index+ip_length).map(|a|response[a] as char).collect::<String>();
            result.push(Ip::new(ip, qtype))
        }
    }


    let next = ip_start_index + ip_length;

    if next >= response.len() {
        Ok(result)
    }
    else {
        result.append( &mut get_ip_r(response, next)?);
        Ok(result)
    }
}

fn format_ipv6(chars: &[char]) -> String {
    //Nach RFC 5952
    let mut res = String::new();
    let mut doubleset = false;
    let mut doubleoff = false;

    for (i, chunk) in chars.chunks(4).enumerate() {
        match chunk.iter().position(|&x| x != '0') {
            Some(v) => {
                res.push_str(&chunk[v..].iter().collect::<String>());
                res.push_str(":");
                if doubleset {
                    doubleoff = true;
                }
            },
            None => {
                if !doubleset {
                    if i == 0 {
                        res.push_str("::");
                    }
                    else {
                        res.push_str(":");
                    }
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
    let bytes = encode(&buf)?;
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
        }?
    }
    Ok(s.chars().map(|a| a as u8).collect::<Vec<u8>>())
}

/// Returns hex of a u8
/// # Arguments
/// * `bytes` - bytes that we want to parse
fn encode(bytes: &[u8]) -> Result<String, CustomError> {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b)?;
    }
    Ok(s)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = get_name_index(3000);
        assert_eq!(x, 12 as usize);
    }
}