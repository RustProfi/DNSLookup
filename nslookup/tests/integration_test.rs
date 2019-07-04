#[cfg(test)]
extern crate nslookup;
use nslookup::qtype::Qtype;
use nslookup::question::{Question, Header, DnsMessageBuilder};
use nslookup::response::{Response, Ip};

#[test]
fn test_qtype() {
    assert_eq!(Qtype::A.value(), 1);
    assert_eq!(Qtype::AAAA.value(), 28);
    assert_eq!(Qtype::CNAME.value(), 5);
    assert_eq!(Qtype::get_qtype(1).unwrap().value(), Qtype::A.value());
    assert_eq!(Qtype::get_qtype(28).unwrap().value(), Qtype::AAAA.value());

    assert_eq!(Qtype::get_qtype(5).unwrap().value(), Qtype::CNAME.value());
}

#[test]
fn test_question() {
    let header = Header::new(1, false, false);
    assert_eq!(header.get_header().unwrap(), b"\x00\x01\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00");

    let question = Question::new("example.com", Qtype::A);
    assert_eq!(question.get_question(), b"\x07\x65\x78\x61\x6d\x70\x6c\x65\x03\x63\x6f\x6d\x00\x00\x01\x00\x01");

    let dns = DnsMessageBuilder::new(header, vec![question]);
    assert_eq!(dns.build_messages().unwrap(), vec![b"\x00\x01\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x07\x65\x78\x61\x6d\x70\x6c\x65\x03\x63\x6f\x6d\x00\x00\x01\x00\x01"])
}

#[test]
fn test_response() {
    let response: Vec<u8> = vec![0, 2, 129, 128, 0, 1, 0, 1, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 28, 0, 1, 192, 12, 0, 28, 0, 1, 0, 0, 1, 19, 0, 16, 42, 0, 20, 80, 64, 1, 8, 32, 0, 0, 0, 0, 0, 0, 32, 14];
    let ips = vec![Ip::new(String::from("2a00:1450:4001:820::200e"), Qtype::AAAA)];
    let hardresult = Response::new(String::from("google.com"), ips);
    assert_eq!(hardresult, Response::parse_response(&response, 28).unwrap());
}
