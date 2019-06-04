# pir-ss19-homeworks-grp7
# nslookup - simple DNS lookup tool written in Rust
Nslookup is a simple command-line tool to query Internet domain name 
servers (DNS). Nslookup has two modes: interactive and non-interactive. 
Interactive mode allows the user to query name servers for information 
about various hosts and domains or to print a list of hosts in a domain.
 Non-interactive mode is used to print just the name and requested 
information for a host or domain. Additionaly it supports both forward 
(domain name to ip) and reverse (ip to domain name) lookups.

## **ARGUMENTS**
Interactive mode is entered in the following cases:
1. when no arguments are given (the default name server will be used)
2. when the first argument is a hyphen (-) and the second argument is 
the host name or Internet address of a name server.

Non-interactive mode is used when the name or Internet address of the 
host to be looked up is given as the first argument. The optional second
argument specifies the host name or address of a name server.

Additional Arguments may follow

## **Crates**
We'll try to implement it using only the standard library the only 
crate we'll consider for now is the nix crate for it's nix::sys::socket 
module.
