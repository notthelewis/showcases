# Considerations 

This file documents potential considerations for various parts of this project. 

### Names

- Name for the software itself could do with workshopping. Blewis is a bit egotistical.

### Security
- Implementation could be vulnerable to a slow-loris attack on decode routine for strings & arrays.
- Zip bombs could be a threat if compression is used
- Implementation could be vulnerable to overflow if decoding nested structures (i.e. array[array[string], string])
    - To mitigate, both encoding and decoding implementations should enforce limits on recursive, unsized elements. 
      This could be exposed as as a configurable parameter, with a sane default chosen (3 levels deep for example)

### Performance
- None-blocking IO may be a big win for performance. I.e. epoll() on Linux, kqueue on BSD. For Rust, Mio or Tokio 
    (which uses mio) handles a lot of the implementation details here. Libuv is a longstanding alternative, written in
    C++ and used by NodeJS
- Object pooling may be a big win too
- Could be a ton of branching logic in the decode routine
- Some of the larger int types (u64, f64) could encode/decode quite poorly on machines with a smaller unit size (<32bit)
- Array could be split into two different encoding formats. If the array is purely numerical and of the same type, a 
  higher entropy encoding format could potentially be devised; using 3 of the padding bits to set the type of the array
  and then the following two bytes for size. Then, each of the data messages could just be sent as they are 
- Exponential read buffer growth might be a big win, potentially reducing reallocations drastically.
- Alternative hashing algorithms could/should be provided for the core data store via command line args / config

## Libraries
- [anyhow](https://docs.rs/anyhow/latest/anyhow/)
    - Simplifies error handling 

- [bytes](https://docs.rs/bytes/latest/bytes/)
    - Used for efficient copies of buffers from net requests 

- [bitreader](https://docs.rs/bitreader/latest/src/bitreader/lib.rs.html#69-77)
    - Used for reading a `&'[u8]` at a bit-level granularity (super convenient)
        - Potentially could add some SIMD... Could be a good side project
