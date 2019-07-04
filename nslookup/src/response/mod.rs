use std::fmt;
use std::vec::Vec;
use crate::customerror::CustomError;
use crate::qtype::Qtype;
use std::net::IpAddr;

/// A struct that holds the Response
#[derive(PartialEq)]
pub struct Response {
    pub domain: String,
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
        write!(f, "Domain: {}\nAdress(es):\n{}", self.domain, res_str)
    }
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut res_str = String::new();
        for ip in &self.ip {
            res_str.push_str(&ip.qtype.to_string());
            res_str.push_str(": ");
            res_str.push_str(&ip.ip);
            res_str.push_str("\n");
        }
        write!(f, "Domain: {}\nAdress(es):\n{}", self.domain, res_str)
    }
}

impl Response {
    /// Returns a new Response
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain
    /// * `ip` - A Vector of IP Structs
    pub fn new(domain: String, ip: Vec<Ip>) -> Self {
        Response{domain, ip}
    }

    /// Tries to parse an DNS-Answer into a Response.
    /// It checks the status of the response.
    /// If it's ok it parses the Domain and Ip(s) from it.
    ///
    /// # Arguments
    ///
    /// * `buf` - The DNS-Answer
    /// * `response_start_index` - The index where the actual respons(es) starts
    pub fn parse_response(buf: &[u8], response_start_index: usize)-> Result<Self, CustomError> {
        if response_start_index >= buf.len() {
            return Err(CustomError::EmptyResponse)
        }
        check_response_status(buf[2], buf[3])?;
        let domain_index_raw = combine_u8tou16(buf[response_start_index], buf[response_start_index + 1]);

        Ok(Response::new(get_domain_r(&buf, get_name_index(domain_index_raw))?, get_ip_r(&buf, response_start_index)?))
    }
}

/// A struct that holds an Ip
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Ip {
    pub ip: String,
    pub qtype: Qtype,
}

impl Ip {
    /// Returns a new Ip
    ///
    /// # Arguments
    ///
    /// * `ip` - The adress
    /// * `qtype` - The query Type
    pub fn new(ip: String, qtype: Qtype) -> Self {
        Ip{ip, qtype}
    }
}

/// Checks a) if the given DNS-Package is acutally a response
/// and b) if errors occured.
///
///+++++++++flag1++++++++++++++++++++flag2+++++++++++++
/// 7  6  5  4  3  2  1     0  7  6  5  4  3  2  1  0
///+--+--+--+--+--+--+--+  +--+--+--+--+--+--+--+--+--+
///|QR|   Opcode  |AA|TC|  |RD|RA|   Z    |   RCODE   |
///
/// QR specifies wether this is a message(0) or response(1). We only want responses.
/// RCODE contains errors and all must be zero.
///
/// # Arguments
///
/// * `flags1` - Contains the flags
/// * `flags2` - Contains the flags
fn check_response_status(flags1: u8, flags2: u8) -> Result<(), CustomError> {
    if !bit_at(flags1, 7) || bit_at(flags2, 3) || bit_at(flags2, 2) ||
        bit_at(flags2, 1) || bit_at(flags2, 0) {
        Err(CustomError::ResponseError)
    }
    else {
        Ok(())
    }
}

/// Returns the value of a single bit in an u8
///
/// # Arguments
///
/// * `input` - The given u8
/// * `n` - The offset
fn bit_at(input: u8, n: u8) -> bool {
    if n < 8 {
        input & (1 << n) != 0
    } else {
        false
    }
}

/// Gets the Domain from the DNS-Package recursively.
/// The domain is dynamic. The first field contains the length of the upcoming domain part,
/// which comes right after the length. The dots are not provided.
/// The field after the first part holds the next length or zero.
/// This repeats until zero occurs.
///
/// Example: google.com (for simplicity the index is 0 in this example)
/// field[0] = 6
/// field[1] until field[7] contains g o o g l e
/// field[8] contains length 3
/// ...
///
/// # Arguments
///
/// * `response` - The complete DNS-Package
/// * `index` - The index where the first length of the domain is located (this is in the question section)
fn get_domain_r(response: &[u8], index: usize) ->  Result<String, CustomError> {
    let length = response[index] as usize;
    let startindex = index + 1;
    let next = startindex + length;

    if next < response.len() {
        let domain_build = (startindex..next).map(|a|response[a] as char).collect::<String>();

        if response[next] == 0 {
            Ok(domain_build)
        }
        else {
            Ok(format!("{}.{}", domain_build, get_domain_r(response, next)?))
        }
    }
    else {
        Err(CustomError::Overflow)
    }
}

/// Gets the Adresses from one or more answers recursively.
/// The answers are concatenated together.
/// An answer contains a query type from which you can know which type of adress the answer contains
/// and of course the adress itself.
/// The function runs until there are no answers left.
/// If a unsupported query type occurs the function throws an error.
///
/// # Arguments
///
/// * `response` - The complete DNS-Package
/// * `answer_start_index` - The index where the first actual answer begins
fn get_ip_r(response: &[u8], answer_start_index: usize) -> Result<Vec<Ip>, CustomError> {
    if answer_start_index + 12 >= response.len() {
        return Err(CustomError::Overflow);
    }

    let mut result = vec![];

    let qtypehex = combine_u8tou16(response[answer_start_index + 2], response[answer_start_index + 3]) as usize;
    let qtype = Qtype::get_qtype(qtypehex)?;

    let ip_length = combine_u8tou16(response[answer_start_index + 10], response[answer_start_index + 11]) as usize;
    let ip_start_index = answer_start_index + 12;

    match qtype {
        Qtype::A | Qtype::AAAA => {
            result.push( Ip::new(format_ip(&response[ip_start_index..ip_start_index + ip_length])?, qtype))
        },
        Qtype::CNAME => {
            if let Ok(cname) =  get_domain_r(&response[ip_start_index..ip_start_index + ip_length], 0) {
                result.push(Ip::new(cname, qtype))
            }

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

/// Formats an array of chars into an ipv6 format by RFC 5952 convention.
///
/// # Arguments
///
/// * `buf` - An array of chars with the hex numbers 16 bytes long.
fn format_ip(buf: &[u8]) -> Result<String, CustomError> {
    if buf.len() == 4 {
        let mut array = [0; 4];
        array.copy_from_slice(buf);
        Ok(IpAddr::from(array).to_string())
    }
    else if buf.len() == 16 {
        let mut array = [0; 16];
        array.copy_from_slice(buf);
        Ok(IpAddr::from(array).to_string())
    }
    else {
        Err(CustomError::IpParseError)
    }
}



/// The response uses a compressed format for the domain name.
/// It's actually just a pointer into the question section.
/// The pointer combined from two u8's contains two leading 1 which
/// have to be shifted away to get the index.
///
/// # Arguments
///
/// * `pointer` - The unshifted pointer from the Answer section.
fn get_name_index(pointer: u16) -> usize {
    (pointer << 2 >> 2) as usize
}

/// Shift operation to combine two u8 into a u16
/// This is necessary because some values are representated by two u8's.
/// Attention: Ordering matters!
///
/// Example: 1+1 = 11
///
/// # Arguments
///
/// * `val1` - The first value.
/// * `val2` - The second value.
fn combine_u8tou16(val1: u8, val2: u8) -> u16 {
    (u16::from(val1) << 8) | u16::from(val2)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_check_response_status() {
        assert_eq!(check_response_status(129 as u8, 128 as u8).unwrap(), (()));
    }

    #[test]
    #[should_panic]
    fn test_check_response_status2() {
        assert_eq!(check_response_status(1 as u8, 128 as u8).unwrap(), (()));
    }

    #[test]
    fn test_bit_at() {
        assert_eq!(false, bit_at(0,0));
        assert_eq!(true, bit_at(1, 0));
    }

    #[test]
    fn test_get_domain_r() {
        let response: Vec<u8> = vec![0, 2, 129, 128, 0, 1, 0, 1, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 28, 0, 1, 192, 12, 0, 28, 0, 1, 0, 0, 1, 19, 0, 16, 42, 0, 20, 80, 64, 1, 8, 32, 0, 0, 0, 0, 0, 0, 32, 14];
        assert_eq!(String::from("google.com"), get_domain_r(&response, 12).unwrap());
    }

    #[test]
    fn test_get_ip_r() {
        let response: Vec<u8> = vec![0, 2, 129, 128, 0, 1, 0, 1, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 28, 0, 1, 192, 12, 0, 28, 0, 1, 0, 0, 1, 19, 0, 16, 42, 0, 20, 80, 64, 1, 8, 32, 0, 0, 0, 0, 0, 0, 32, 14];
        let ip = Ip::new(String::from("2a00:1450:4001:820::200e"), Qtype::AAAA);
        let ips = vec![ip];
        assert_eq!(get_ip_r(&response, 28).unwrap(), ips);
    }

    #[test]
    fn test_format_ipv6() {
        let vec1 = "20010000000000000001000000000001".chars().collect::<Vec<char>>();
        let vec2 = "2a0014504001081c000000000000200e".chars().collect::<Vec<char>>();
        let vec3 = "00000000000000000000ffff40bc3892".chars().collect::<Vec<char>>();
        assert_eq!(format_ipv6(&vec1), String::from("2001::1:0:0:1"));
        assert_eq!(format_ipv6(&vec2), String::from("2a00:1450:4001:81c::200e"));
        assert_eq!(format_ipv6(&vec3), String::from("::ffff:40bc:3892"));
    }

    #[test]
    fn test_get_name_index() {
        assert_eq!(get_name_index(49164), 12);
    }

    #[test]
    fn test_combine_u8tou16() {
        assert_eq!(combine_u8tou16(181, 180), 46516)
    }

    #[test]
    fn test_encode() {
        let response: Vec<u8> = vec![0, 2, 129, 128, 0, 1, 250];
        assert_eq!(encode(&response).unwrap(), String::from("000281800001fa"));
    }
}