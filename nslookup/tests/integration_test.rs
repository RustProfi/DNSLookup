#[cfg(test)]
extern crate nslookup;
use nslookup::qtype::Qtype;
use nslookup::question;
use nslookup::question::{Question, Header};

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
    let header = Header::new_message(1,false, false);
    assert_eq!(header.unwrap(),b"\x00\x01\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00");
    let header2 = Header::new_message(2,false, false);
    assert_eq!(Question::new_question(header2.unwrap(),"example.com", Qtype::A), b"\x00\x02\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x07\x65\x78\x61\x6d\x70\x6c\x65\x03\x63\x6f\x6d\x00\x00\x01\x00\x01")
}