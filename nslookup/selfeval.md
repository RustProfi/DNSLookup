# Self evaluation

## Neutral
We didn't use any external crates which was our goal from the beginning. For example important conversions like hex <-> bytes or binary -> hex where implemented by us 
(crates like hex or Binary exist). We also wrote our own parser that supports the Record Types A(Ipv4), AAAA(Ipv6) and CNAME(canonical name). 
Other Record Types (which there are over 50 of) aren't supported due to a lack of time. 

We could have used the to_socket_addrs method in
std::nt::TosocketAddrs for a simple generic address resolution. But this would have done all the work for us.

A DNS Parser library would have helped but would also made implementation trivial but also more bloated because
a lot of Record types are deprecated or aren't widely used. 

## Good
For Error Handling we use a CustomError including self definded Error types which builds a wrapper around already existing types (e.g. std::io::Error) by implementing the From trait what makes it very easy to use due to the ? operator.

We build up a well thought out data structure for our use case for very easy use. Additionally we implemented Display and Debug for it.

We also made the code idiomatic as possible using functional paradigms and methods.

We have integration & unit tests and also we made sure to document everything.

We made use of bitwise shift operations which happen on the stack for more performance.

We used PartialEq and Debug Macros.

## Bad
We didn't reach our goal to implement interactive mode and reverse lookup. Interactive mode was left behind because of 
time constrains and it would have gone beyond the scope. The reason why reverse lookup was removed is because we used the Inverse querie method that has been deprecated since 2002.
The DNS documentation wasn't immediatly clear on this. The guaranteed way to get a correct mapping is by using a IN-ADDR.ARPA domain.correct reverse ip response.
