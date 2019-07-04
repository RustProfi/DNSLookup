# Self evaluation

## Good and Neutral
We didn't use any external crates which was our goal from the beginning. For example important conversions like hex <-> bytes or binary -> hex where implemented by us 
(crates like hex or Binary exist). We also wrote our own parser that supports the Qtypes A, AAAA and CNAME. We could have used the to_socket_addrs method
std::nt::TosocketAddrs for a simple generic address resolution. But this would have made it too easy. We only support A, AAAA and CNAME
other Record Types (which there are over 50 of) aren't supported. A DNS Parser library would have helped but would also make implementation trivial but also more bloated because
a lot of Record types are deprecated or aren't widely used. Another good thing is our CustomError type which helps make the code cleaner because of the use of the ? operator.
We also tried making the code idiomatic as possible using functional paradigms and methods. In some places you could replace for example the for loop with an iterator
but we couln't make it work with Error handling.

## Bad
We didn't reach our goal to implement interactive mode and reverse lookup. Interactive mode was left behind because of 
time constrains. The reason why reverse lookup was removed is because we used the Inverse querie method that has been deprecated since 2002.
The DNS documentation wasn't immediatly clear on this. The guaranteed way to get a correct mapping is by using a IN-ADDR.ARPA domain.correct reverse ip response.