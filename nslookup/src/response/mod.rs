use std::fmt::Write;
use std::fmt;
use std::vec::Vec;
use crate::customerror::CustomError;
use crate::qtype::Qtype;

/// A struct that holds the Response
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

impl Response {
    /// Returns a new Response
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain
    /// * `ip` - A Vector of IP Structs
    fn new(domain: String, ip: Vec<Ip>) -> Self {
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
            return Err(CustomError::EmptyResponse);
        }
        check_response_status(&to_binary_vec(&buf[2..4])?)?;
        let domain_index_raw = combine_u8tou16(buf[response_start_index], buf[response_start_index + 1]);

        Ok(Response::new(get_domain_r(&buf, get_name_index(domain_index_raw)), get_ip_r(&buf, response_start_index)?))
    }
}

/// A struct that holds the Response
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
    fn new(ip: String, qtype: Qtype) -> Self {
        Ip{ip, qtype}
    }
}

/// Checks a) if the given DNS-Package is acutally a response
/// and b) if errors occured.
///
/// The first bit must be 1 for a response
/// The last four bits must be 0 for no errors.
///
/// # Arguments
///
/// * `bits` - The status bits in an array
fn check_response_status(bits: &[u8]) -> Result<(), CustomError> {
    if bits.len() < 16 || bits[0] == 0 || bits[12] == 1 || bits[13] == 1 || bits[14] == 1 || bits[15] == 1 {
        Err(CustomError::ResponseError)
    }
    else {
        Ok(())
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
fn get_domain_r(response: &[u8], index: usize) -> String {
    let length = response[index] as usize;
    let startindex = index + 1;
    let next = startindex + length;
    let domain_build = (startindex..startindex+length).map(|a|response[a] as char).collect::<String>();
    if response[next] == 0 {
        domain_build
    }
    else {
        format!("{}.{}", domain_build, get_domain_r(response, next))
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
    let mut result = vec![];

    let qtypehex = combine_u8tou16(response[answer_start_index + 2], response[answer_start_index + 3]) as usize;
    let qtype = Qtype::get_qtype(qtypehex)?;

    let ip_length = combine_u8tou16(response[answer_start_index + 10], response[answer_start_index + 11]) as usize;
    let ip_start_index = answer_start_index + 12;

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


/// Formats an array of chars into an ipv6 format by RFC 5952 convention.
///
/// # Arguments
///
/// * `chars` - An array of chars with the hex numbers.
fn format_ipv6(chars: &[char]) -> String {
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