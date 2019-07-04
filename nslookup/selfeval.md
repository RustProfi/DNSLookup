# Self evaluation

## Good
No external crates used.  For example important conversions like hex <-> bytes or binary -> hex where implemented by us 
(crates like hex or Binary exist). We also wrote our own parser that supports the Qtypes A,AAAA and CNAME. We have to admit that it's not perfect
as oppose to many DNS Parser crates just for the simple fact that we don't have the time to support over 50 different Record types.

DNS Parsers crates would 
They generally contain a lot of old Record types

Also we didn't use the std::nt::TosocketAddrs::to_socket_addrs(&self) method for generic address resulution

CustomError use of ? operator
## Bad
No interactive mode and it only supports forward lookups.
We read an outdated DNS documentation