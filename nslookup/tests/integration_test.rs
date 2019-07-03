#[cfg(test)]
extern crate nslookup;
use nslookup::qtype::Qtype;

#[test]
fn test_qtype() {
    assert_eq!(Qtype::A.value(), 1);
    assert_eq!(Qtype::AAAA.value(), 28);
    assert_eq!(Qtype::CNAME.value(), 5);
    assert_eq!(Qtype::get_qtype(1).unwrap().value(), Qtype::A.value());
    assert_eq!(Qtype::get_qtype(28).unwrap().value(), Qtype::AAAA.value());
    assert_eq!(Qtype::get_qtype(5).unwrap().value(), Qtype::CNAME.value());
}