use std::env;
use std::net::{ToSocketAddrs, SocketAddr, UdpSocket};
use std::vec::IntoIter;
use std::io::Error;

fn main() {
    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && &args[1] != "-help" {
        if check_ip(&args[1]) {
            println!("reverse lookup")
        } else {
            match get_address(&args[1]) {
                Ok(iter) => {
                    for x in iter {
                        x //sock.send_to(,x)
                    }
                }
                Err(e) => println!("{:?}", e),
            };

        }
    } else {
        println!("Usage is: nslookup [Host Name] | [Host IP] | -help");
        println!("nslookup foo.bar.com (Returns IP Address for Host Name)");
        println!("nslookup 123.123.123.123 (Returns Host Name for Address)");
        println!("nslookup -help (Returns this Help Message)");
    }
}

fn get_address(domain: &str) -> Result<IntoIter<SocketAddr>, Error> {
    format!("{}:53", domain).to_socket_addrs()
}




fn check_ip(ip: &str) -> bool {
    if ip.split(".").count() == 4 {
        ip.split(".").all(|s| s.parse::<i32>().is_ok())
    } else {
        false
    }
}
