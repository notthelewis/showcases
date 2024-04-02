# BOOP 

_Bidirectional.Ostensibly.Optimal.Protocol_

# Overview

Since I'm clearly a masochist, I want to build my own protocol for this project.

I'm going to encode data types with lengths pre-determined ahead of time. This prevents the need for sending one byte or
more in order to specify the length of every field. Simple data types, such as integers, have a known size at encode /
decode time, so this information does not need to be specifed for every message. Alongside this, this removes the need 
for control sequences completely. I am not using Serde for this project, as I clearly stated I'm a masochist... Plus I
want to keep dependencies and compile times down to an absolute minimum. I understand the things I'm missing out on by 
choosing not to use Serde; The biggest drawback being future-proofing the protocol. I cannot predict the future of how 
this software might be used so I likely won't get every detail right. There are _some_ measures taken to reduce this 
issue but primarily the requirements are quite static. It's a remote-controlled concurrent hashmap with an extremely 
simple data format.

Since this project is effectively a simplified Redis, I'll draw some comparisons between `RESP3` and `BOOP`, then 
explain the design differences.

Redis states:
> RESP is a compromise among the following considerations:
> 
> Simple to implement.
> Fast to parse.
> Human readable.

`BOOP`'s operational objectives vary considerably from `RESP`'s. It doesn't care about being human readable. The main 
things reading the protocol aren't going to be humans; They're going to be computers. Tools can be made to print the 
protocol out in readable ways, if such a thing is needed. Standard tools like xxd, hexdump or wireshark already give a
solid foundation if you understand the protocol intrinsics.

Removing the human readable compromise, `BOOP` can focus more on the implementation being simple(ish) to implement & 
efficient to encode and decode... Whilst aiming for optimal wire size. I've gone for the approach of maximum entropy. 
If, later on down the line, I wanted to modify the protocol it could mean large modifications. I'm willing to take that 
risk on in order to ensure that data is encoded in as little space as possible. This is because I'm aiming to make 
`Blewis` optimal for low bandwidth scenarios like a congested dial-up internet connection, or low-power GSM modems on 
embedded devices. One step that can be taken to reduce the impact of future iterations is to have a handshake process,
which allows for a protocol version to be agreed upon by server and client.

I'm tempted to use some kind of compression for large strings / arrays, which could be really useful for low bandwidth
situations but may be counter productive for embedded devices. If I do use compression, I'm thinking of using Facebook's
`Zstandard`, as it is efficient to encode & decode and provides truly excellent entropy. They also claim to have 
excellent small data compression too... which might be useful. I'm not yet sold on whether the compute required 
warrants the perceived benefit of compression, as the time taken to compress, encode, decompress and decode might 
negate the benefits of the smaller format.


# Network layer

Since this is a Redis clone, the model is going to be client-server. Unlike `RESP`, `BOOP` will support _multiple_ 
protocols at the network layer, natively. It will support:

1) TCP
2) UDP
3) Unix sockets
4) HTTP
5) WebSockets

## TCP
TBD. 

## UDP
TBD. 

## Unix sockets
TBD. 

## HTTP
TBD. 

## WebSockets
TBD. 

# Spec

Similarly to `RESP3`, messages will be passed between client & server via `Frames`. However, the framing of messages 
in `BOOP` differs wildly to framing of messages in `RESP`. In `RESP`, every message that is sent is prepended with a 
header byte and suffixed with a `\r\n` control sequence.

In `BOOP`, messages are going to be prepended with meta data and that's it. No suffix is needed. The main benefit being 
the ability to have full access to every possible combination of bytes. If one wanted to encode any kind of binary data,
this is a big win. Values don't have to be escaped like they would with a control character. In an embedded device with
realtime data requirements this might save many wasted clock cycles validating data before it is sent, as opposed to 
just encoding and sending it. 

## Framing

The way that data is actually framed is Type Length Value, except the length is omitted for data types that have a 
known value at compile time; namely ints, floats & bools. The data on its own is kind of meaningless though. This is 
where commands come in. Like with Redis, clients have the ability to store arbitrary data with arbitrary keys and 
retrieve those values later. Unlike Redis, however, `Blewis` is much less equipped to deal with some of advanced 
use-cases such as programmability, advanced queries, geospatial data and a few others. 

As a result, the command set is much smaller. We have:
 - Get:   retrieve a value 
 - Set:   update, remove or insert a value
 - Pub:   push updates to a topic
 - Sub:   receive updates from a topic
 - Inc:   increment value 
 - Dec:   decrement value

The first byte of every message sent from a client->server should be a command. Since there's only 7 commands total, we
can encode this in 3 bits (MSB).

```
|---------!-----------|
|         !  position |
|---------!-----------|
|  type   ! 0 ! 1 | 2 |
!=========!===!===!===|
|   Get   ! 0 ! 0 ! 0 |
|---------!---!---!---|
|   Set   ! 1 ! 0 ! 0 |
|---------!---!---!---|
|   Pub   ! 0 ! 1 ! 0 |
|---------!---!---!---|
|   Sub   ! 0 ! 0 ! 1 |
|---------!---!---!---|
|   Inc   ! 1 ! 1 ! 0 |
|---------!---!---!---|
|   Dec   ! 1 ! 1 ! 1 |
|---------!---!---!---|
```

This leaves 5 bits left in the byte, which can either be padded to zeros or utilised as a bitmap. 

### GET command

#### GET with no flags 

```
|-------------------------------|
|             position          |
|===========|-------------------|
| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|===|===|===|===!===!===!===!===|
| 0 | 0 | 0 | 0 ! 0 ! 0 ! 0 ! 0 |
|===========|-------------------|
|    GET    |
|-----------|
```
1) Retrieves a value if it exists
2) Changes no meta data 
3) Replies with a single `DataType` with zero extra framing. See `Data Types` for more information.

Text command structure:
>> GET $keyname

#### GET with delete (GETDEL)
```
|-------------------------------|
|             position          |
|===========|-------------------|
| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|===|===|===|===!===!===!===!===|
| 0 | 0 | 0 | 1 ! 1 ! 1 ! 1 ! 1 |
|===========|---!---!---!---!---|
|    GET    |       DELETE      |
|-----------|-------------------|
```
1) Retrieves a value if it exists
2) Deletes entry if one is found
3) Replies with a `BoopError` data type, with a 0 code if successful and other error codes TBD

Text command structure:
>> GETDEL $keyname

#### GET with set (GETSET)

```
|-------------------------------|
|             position          |
|===========|-------------------|
| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|===|===|===|===!===!===!===!===|
| 0 | 0 | 0 | 0 ! 0 ! 0 ! 0 ! 1 |
|===========|---!---!---!---!---|
|    GET    |        SET        |
|-----------|-------------------|
```
1) Retrieves a value if it exists
2) Sets the value at key regardless of whether one previously existed
3) Returns the previous value if one was present, otherwise will return a `BoopError`

Text command structure: 
>> GETSET $keyname $newvalue

### SET command

```
|-------------------------------|
|             position          |
|===========|-------------------|
| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|===|===|===|===!===!===!===!===|
| 0 | 0 | 1 | 0 ! 0 ! 0 ! 0 ! 0 |
|===========|---!---!---!---!---|
|    SET    |
|-----------|
```

1) Sets a value at the given key.
2) If an entry already existed, update it and return the old value. 
    Otherwise, create a new value and return it upon completion.

### PUB command

### SUB command
### INC command
### DEC command


# Data types

I'm only supporting the following data types;
1) integer
2) bool
3) string
4) error
5) array

All data types are encoded MSB (Big Endian).

Every encoded message is prepended with variable length meta data. The first 3 bits follow this format,
then each data type is encoded differently.

```
|---------!-----------|
|         !  position |
|---------!-----------|
|  type   ! 0 ! 1 | 2 |
!=========!===!===!===|
| integer ! 0 ! 0 ! 0 |
|---------!---!---!---|
| bool    ! 0 ! 0 ! 1 |
|---------!---!---!---|
| string  ! 0 ! 1 ! 0 |
|---------!---!---!---|
| error   ! 1 ! 0 ! 0 |
|---------!---!---!---|
| array   ! 1 ! 1 ! 0 |
|---------!---!---!---|
```

### Integer

I will support the following types of integer:
 - tiny (u8)                0x00
 - small (u16)              0x08
 - medium (u32)             0x10
 - large (u64)              0x20
 - floating_small (f32)     0x30
 - floating_large (f64)     0x38

We can represent this with 3 bits. Remember, all data is encoded MSB

```
|============!===========|
|            !  bit pos  |
|------------!-----------|
|  int type  ! 2 ! 3 ! 4 |
|============!===!===!===|
|tiny        ! 0 ! 0 ! 0 |
|------------!---!---!---|
|small       ! 0 ! 0 ! 1 |
|------------!---!---!---|
|medium      ! 0 ! 1 ! 0 |
|------------!---!---!---|
|large       ! 1 ! 0 ! 0 |
|------------!---!---!---|
|floating_S  ! 1 ! 1 ! 0 |
|------------!---!---!---|
|floating_L  ! 1 ! 1 ! 1 |
|------------!---!---!---|
```

This gives us the nice, predictable quality that: 
>> An int's meta data always takes up 6 bits, regardless of the type of integer

```
  7   6   5   4   3   2
|---!---!---!---!---|---|
| 0 ! 0 ! 0 ! 0 ! 0 ! 0 | ---------> tiny integer meta data
|---!---!---!---!---!---|

  7   6   5   4   3   2
|---!---!---!---!---|---|
| 0 ! 0 ! 0 ! 0 ! 0 ! 1 | ---------> small integer meta data
|---!---!---!---!---!---|

  7   6   5   4   3   2
|---!---!---!---!---|---|
| 0 ! 0 ! 0 ! 0 ! 1 ! 0 | ---------> medium integer meta data
|---!---!---!---!---!---|

  7   6   5   4   3   2
|---!---!---!---!---|---|
| 0 ! 0 ! 0 ! 1 ! 0 ! 0 | ---------> large integer meta data
|---!---!---!---!---!---|

  7   6   5   4   3   2
|---!---!---!---!---|---|
| 0 ! 0 ! 0 ! 1 ! 1 ! 0 | ---------> floating_small integer meta data
|---!---!---!---!---!---|

  7   6   5   4   3   2
|---!---!---!---!---|---|
| 0 ! 0 ! 0 ! 1 ! 1 ! 1 | ---------> floating_large integer meta data
|---!---!---!---!---!---|
```

The remaining 2 bits will be padded to zero.

This means that the implementation is (relatively) simple. If encoding a u8, we need 1 more byte. If it's a u16 we need 
two more, u32 or f32 we need 4 more bytes and if it's a u64 or f64, we need 8 more bytes. Basically, we can encode 
_nearly_ all the major integer types (no 2s compliment) in N+1 bytes, where N == len of data type itself. Let's compare
this to how RESP3 handles the encoding of numbers. 

Here's the encoding of the number 3735928559 (0xDEADBEEF) in the RESP3 protocol:
```
| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 |   <--- NOTE: BYTES not BITS
|---!---!---!---!---!---!---!---!---!---!----!----!----|
| , ! 3 ! 7 ! 3 ! 5 ! 9 ! 2 ! 8 ! 5 ! 5 ! 9  ! \r ! \n |
|---!---!---!---!---!---!---!---!---!---!----!----!----|
```

And here's the encoding of the same number in my version:
```
|  0   |   1  |   2  |   3  |  4   |    <--- BYTES not its
| 0x04 | 0xDE | 0xAD | 0xBE | 0xEF |
```

This means that this new binary protocol uses 5 bytes instead of 13. That's 62% smaller than the original encoding. Now,
it's worth mentioning here that there is a benefit to RESP3 which is as of yet unaccounted for; which is the number of
system calls to `read()`. If we were reading in the traditional sense yes, our method would incurr more system calls.

However, we're using ✨Rust✨, which is a modern language that, along with all other modern languages, has the concept 
of a BufferedReader baked right in. So, we're not going to just be reading byte-by-byte. Our buffered reader is going to 
optimise our number of reads from the get-go. So even if the implementation `Read`s twice, it doesn't necessarily call
the `read()` syscall twice.

And the final nail in the coffin, for me, is that we need far fewer allocations and overall operations during the parse 
routine. When parsing the RESP3 version, a naive routine _could_ look something like this:

```rs
let mut read_buf: Vec<u8> = Vec::with_capacity(MAX_BULK_STRING_LEN);
reader
    .read_until(b'\n', &mut read_buf)
    .context("should read upto newline")?;

// Validate \r\n exists properly
if let Some(newline) = read_buf.pop() {
    if newline != b'\n' {
        return Err(anyhow::anyhow!("expected last entry to be newline"));
    }
}
if let Some(cr) = read_buf.pop() {
    if cr != b'\r' {
        return Err(anyhow::anyhow!("expected last entry to be carriage return"));
    }
}

if let Some(msg_type) = read_buf.get(0) {
    // Maybe u32 assumption isn't great but this is just a simple example
    let int: u32 = String::from_utf8(read_buf[1..])
        .with_context(|| "unable to convert {read_buf} to valid utf-8")?
        .parse()
        .context("unable to parse string: {read_buf} to u32")?;
    // Do something with parsed integer
}

```

The amount of allocations required to do such a simple thing as to read a number from a buffer here are way too 
substantial. There are more efficient ways of doing this (especially if you don't mind unsafe) but the problem with 
encoding integers like this is the amount of unnecessary allocations and overalll space required to encode and decode 
numbers. There is the `atoi` crate, which allows integers to be parsed from ascii, which is likely a more efficient 
solution. That requires taking a dependency though, and the conversion is still not _free_. Another approach could be:
```rs
//          
//          ascii: [6,  4,  4 ]
let read_buf = vec![54, 52, 52];
let num: i32 = read_buf.iter().rev().enumerate().map(|(idx, val)| {
    (val - 48) * 10.pow(idx) 
}).sum();

assert_eq!(num, 644);
```

The way to do this in the BOOP would be something like this:

```rs
let mut read_buf: Vec<u8> = Vec::with_capacity(MAX_LINE_LEN);
bufreader
    .read_exact(&mut read_buf[..2])
    .context("should read meta data")?;

// Get first 6 bits
let meta_data: u8 = &read_buf[..1];
match meta_data {
    // If we're handling a u32
    0x10 => {
        bufreader
            .read_exact(4)
            .context("should read 4 bytes")?;
        let int_u32 = u32::from_ne_bytes(read_buf[2..].try_into().unwrap())
            .context("should convert 4 bytes to u32")?;
    }
}
```

Now, in practice there would be a lot of things done differently. That being said, we can see that there are substantially 
fewer allocations made than the equivalent RESP3 parse routine. Not counting any allocations occuring at the buffered 
reader, we can see that we allocate:
 1) the vector itself upon the first read call (stack first then heap on push)
 2) the meta_data variable (a u8), which does add branching logic, increasing the chance of branch misprediction
 3) the int_u32 variable. 

This is 3 allocations, two of which are stack allocated (meta_data & int_u32) and the heap allocation for the read 
buffer.

The read routine for the RESP encoded u32 requires:
 1) the read_buf vector upon the first read call (stack first then heap on push)
 2) two stack allocations (and copies) for the two u8 values that should contain \r\n and subsequently this introduces 
    four branches, which adds three more chances for branch misprediction
 3) a heap allocated string, with another branch added for when the string is not valid utf-8
 4) however many allocations the standard `String::parse::<usize>()` function makes
 5) finally, a stack allocated u32

In conclusion, there are far fewer allocations required with this proposed protocol, and most of the allocations made 
reside on the stack, which is far more efficient than going through the allocator to store short-lived, tiny data on the
heap. Granted, there may be a better way of doing the first method, though it's still not going to get close to data which
is encoded in the format nearly identical to what most sane programming languages already expect. 

### Floating point integers
Floating point will be encoded as IEEE-754, with single precision for f32 and double precision for f64. I won't put many
words on the subject, as this will be handled almost entirely by the implementation language and is a clear, defined 
standard with many resources readily available online.

### Bool

A bool is the easiest data type to implement. Since there's only ever 2 states, we can encode all of this state in just 
a single bit. 0 == false, 1 == true. This means that we can realistically encode a bool within the meta_data byte.

```
|===========|===================|
| 7 ! 6 ! 5 | 4 ! 3 ! 2 ! 1 ! 0 |
|---!---!---|---!---!---!---!---|
| 0 ! 0 ! 1 | 0 ! 0 | 0 | 0 | 1 |
|-----------|---------------|---|
| Type meta |<-- padding -->| ^ |
|===========|               | 1 == true, 0 == false
```

This is a highly efficient encoding. Not least of all because it's just one byte but also because the decoding of it is 
extremely simple to implement and efficient to execute. It can be achieved one or two bitwise `AND` operations, 
implementation dependent. Or, we could just use it within a big match or switch-case block.

### String

Strings are just going to be utf-8 encoded byte arrays, with a length prepended in a u16 directly after the meta data.
This means that the maximum string length is 65535. This means that every string that is encoded is N+2 bytes 
(exc meta_data), where N is the number of UTF8 encoded bytes the string contains. Of course, this isn't good entropy 
if we're only encoding tiny strings but the trade off is well worth it, as two bytes is practically zero when encoding
Hamlet.

### Error

An Error message is, for all intents and purposes, one u8 and a String (after the meta data byte). One quirk that I'm 
encoding though is that the last bit of the meta data block will be set if the error is a server error and unset if it's
a client error. The u8 after the meta byte is going to contain an error code (0-255). allowing for 512 possible errors 
to be encoded. Finally, the error message will just be a length prepended string, following the exact same convention as
the string data type. 

### Array

The encoding for the Array type is designed to be simple to decode and encode, whilst still being efficient in terms of 
both entropy and implementation. After the meta data byte, 2 bytes will be sent which indicates the number of array 
elements which are to follow. Each element is then one of any of the other data types, prepended with their respective 
header.

For example, an array containing a single u8 of value 256 would be encoded like so:

```
            |     end of byte 0 |                 end of byte 1 |                 end of byte 2 |                 end of byte 3 |                 end of byte 4 |
|===!===!===|---!---!---!---!---|---!---!---|---!---!---!---!---!---!---!---|---!---!---!---!---|---!---!---|---!---!---!---!---|---!---!---|---!---!---!---!---| 
| 1 ! 1 ! 0 | 0 ! 0 ! 0 ! 0 ! 0 | 1 ! 0 ! 0 | 0 ! 0 ! 0 ! 0 ! 0 ! 0 ! 0 ! 0 | 0 ! 0 ! 0 ! 0 ! 0 | 0 ! 0 ! 0 | 0 ! 0 ! 0 ! 0 ! 0 | 1 ! 1 ! 1 | 1 ! 1 ! 1 ! 1 ! 1 |
|===!===!===!---!---!---!---!---|---!---!---!---!---!---!---!---|---!---!---!---!---!---!---!---|---!---!---!---!---!---!---!---|---!---!---!---!---!---!---!---|
|   array   |  <-- padding -->  |                         length (1)                            |   u8                          |   value(256)                  |

```
