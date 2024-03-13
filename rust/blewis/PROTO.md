# Protocol

Since I'm clearly a massochist, I want to build my own protocol for this project. I'm not going to follow the usual TLV 
encoding that I'm familiar with. I'm going to encode data types with pre-determined lengths ahead of time. This prevents
the need for sending one byte or more in order to specify the length of every field. Simple data types, such as integers,
will have a known size at encode/decode time, so this information does not need to be specifed for every message. I will
do some bit-packing and such.

# Spec

I'm only supporting the following data types;
1) integer
2) bool
3) string
4) error
5) array
6) map

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
| map     ! 1 ! 1 ! 1 |
|---------!---!---!---|
```
The reason for this format is because the encode and decode implementations will be _almost_ branchless, regardless
of whatever is being chosen. There are a couple down sides to this approach. The first and foremost being protocol flexibility.

I've gone for the approach of maximum entropy; if later on down the line I wanted to modify the protocol, it would likely
mean large modifications 

## Integer

I will support the following types of integer:
 - tiny (u8)
 - small (u16)
 - medium (u32)
 - large (u64)
 - floating_small (f32)
 - floating_large (f64)

We can represent this with 3 bits. 

```
|------------!-----------!
|  int type  !    pos    |
|------------!---!---!---|
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
  0   1   2   3   4   5
|---!---!---!---!---|---|
| 0 ! 0 ! 0 ! 0 ! 0 ! 0 | ---------> tiny integer meta data
|---!---!---!---!---!---|

  0   1   2   3   4   5
|---!---!---!---!---|---|
| 0 ! 0 ! 0 ! 0 ! 0 ! 1 | ---------> small integer meta data
|---!---!---!---!---!---|

  0   1   2   3   4   5
|---!---!---!---!---|---|
| 0 ! 0 ! 0 ! 0 ! 1 ! 0 | ---------> medium integer meta data
|---!---!---!---!---!---|

  0   1   2   3   4   5
|---!---!---!---!---|---|
| 0 ! 0 ! 0 ! 1 ! 0 ! 0 | ---------> large integer meta data
|---!---!---!---!---!---|

  0   1   2   3   4   5
|---!---!---!---!---|---|
| 0 ! 0 ! 0 ! 1 ! 1 ! 0 | ---------> floating_small integer meta data
|---!---!---!---!---!---|

  0   1   2   3   4   5
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
|  0   |   1  |   2  |   3  |  4   |    <--- BYTES not Bits
| 0x04 | 0xDE | 0xAD | 0xBE | 0xEF |
```

This means that this new binary protocol uses 5 bytes instead of 13. That's 62% smaller than the original encoding. Now,
it's worth mentioning here that there is a benefit to RESP3 which is as of yet unaccounted for; which is the number of
system calls to `read()`. If we were reading in the traditional sense yes, our method would incurr more system calls.

However, we're using ✨Rust✨, which is a modern language who, along with all other modern languages, has the concept of 
a BufferedReader baked right in. So, we're not going to just be reading byte-by-byte. Our buffered reader is going to 
optimise our number of reads from the get-go. So even if the implementation `Read`s twice, it doesn't necessarily call
the syscall: `read()` twice.

And the final nail in the coffin, for me, is that we need far fewer allocations and overall operations during the parse 
routine. When parsing the RESP3 version, the routine might look something like this:

```rs
let mut read_buf: Vec<u8> = Vec::with_capacity(MAX_BULK_STRING_LEN);
reader
    .read_until(b'\n', &mut read_buf)
    .context("should read upto newline")?;

// Validate \r\n exists properly
if let Some(newline) = vec_ptr.pop() {
    if newline != b'\n' {
        return Err(anyhow::anyhow!("expected last entry to be newline"));
    }
}
if let Some(cr) = vec_ptr.pop() {
    if cr != b'\r' {
        return Err(anyhow::anyhow!("expected last entry to be carriage return"));
    }
}

let int: u32 = String::from_utf8(read_buf)
    .with_context(|| "unable to convert {read_buf} to valid utf-8")?
    .parse()
    .context("unable to parse string: {read_buf} to u32")?;

read_buf.clear();
```

The amount of allocations required to do such a simple thing as to read a number from a buffer here are way too 
substantial. Perhaps there's a more efficient, unsafe way of doing just that but it's not immediately obvious to me.

The way to do this in the encoding format I'm defining would be something like this:

```rs
// NOTE: This won't quite compile, there's a couple things need changing- specifically around the bitshifting etc
let mut read_buf: Vec<u8> = Vec::with_capacity(MAX_LINE_LEN);
reader
    .read_exact(&mut read_buf[..1])
    .context("should read meta data")?;

// Get first 6 bits
let meta_data: u8 = &read_buf[..1] & 0b_111_111__00;
match meta_data {
    // If we're handling a u32
    0b_000_010 => {
        reader
            .read_exact(4)
            .context("should read 4 bytes")?;
        let int_u32 = u32::from_ne_bytes(read_buf[2..])
            .context("should convert 4 bytes to u32")?;
    }
}
```

Now, in practice there would be some things done differently. That's not even tested code. I'm almost positive that the
meta_data slice and substant & operation won't actually work like that. That being said, we can see that there are 
substantially fewer allocations made than the equivalent RESP3 parse routine. Not counting any allocations occuring at 
the buffered reader, we can see that we allocate:
 1) the vector itself upon the first read call (stack first then heap on push)
 2) the meta_data variable (a u8), which adds branching logic, increasing the chance of branch misprediction
 3) the int_u32 variable. 

This is 3 allocations, two of which are stack allocated (meta_data & int_u32) and the heap allocations would occur in the
inverse case anyhow. 

The read routine for the RESP encoded u32 requires:
 1) the read_buf vector upon the first read call (stack first then heap on push)
 2) two stack allocations (and copies) for the two u8 values that should contain \r\n and subsequently this introduces 
    four branches, which adds three more chances for branch misprediction
 3) a heap allocated string, with another branch added if the string is not valid utf-8
 4) however many allocations the standard `String::parse::<usize>()` function makes
 5) finally, a stack allocated u32

In conclusion, there are far fewer allocations required with this proposed protocol, and most of the allocations made 
reside on the stack, which is far more efficient than going through the allocator to store short-lived, tiny data on the
heap. Granted, there may be a better way of doing the first method, though it's still not going to get close to data which
is encoded in the format that the language is already expecting.


### Floating point integers
Floating point will be encoded as IEEE-754, with single precision for f32 and double precision for f64. I won't put many
words on the subject, as this will be handled almost entirely by the implementation language and is a clear, defined 
standard with many resources readily available online.

## Bool

A bool is the easiest data type to implement. Since there's only ever 2 states, we can encode all of this state in just 
a single bit. 0 == false, 1 == true. This means that we can realistically encode a bool in just 1 byte.

```
|===========|===================|
| 0 ! 1 ! 2 | 3 ! 4 ! 5 ! 6 ! 7 |
|---!---!---|---!---!---!---!---|
| 0 ! 0 ! 1 | 0 ! 0 | 0 | 0 | 1 |
|-----------|---------------|---|
| Type meta |<-- padding -->| ^ |
|===========|               | 1 == true, 0 == false
```

This is a highly efficient encoding. Not least of all because it's just one byte but also because the decoding of it is 
extremely simple to implement and efficient to execute. It can be achieved one or two bitwise `AND` operations, 
implementation dependent.

## String

Strings are just going to be utf-8 encoded byte arrays, with a length prepended in a u16 directly after the meta data.
This means that the maximum string length is 65535. This means that every string that is encoded is N+2 bytes, where N
is the number of UTF8 encoded bytes the string contains. Of course, this isn't the entropy if we're only encoding tiny 
strings but the trade off is well worth it, as two bytes is practically zero. 

## Error

An Error message is, for all intents and purposes, one u8 and a String (after the meta data byte). One quirk that I'm 
encoding though is that the last bit of the meta data block will be set if the error is a server error and unset if it's
a client error. The u8 after the meta byte is going to contain an error code (0-255). allowing for 512 possible errors 
to be encoded. Finally, the error message will just be a length prepended string, following the exact same convention as
the string data type. 

## Array

The encoding for the Array type is designed to be simple to decode and encode, whilst still being efficient in terms of 
both entropy and implementation. After the meta data byte, 2 bytes will be sent which indicates the number of array 
elements which are to follow. Each element is then one of any of the other data types, prepended with their respective 
header.

For example, an array containing a single u8 of value 256 would be encoded like so:

```
            |     end of byte 0 |                 end of byte 1 |                 end of byte 2 |                 end of byte 3 |                 end of byte 4 |
|===!===!===|---!---!---!---!---|---!---!---|---!---!---!---!---!---!---!---|---!---!---!---!---|---!---!---|---!---!---!---!---|---!---!---|---!---!---!---!---| 
| 1 ! 1 ! 0 | 0 ! 0 ! 0 ! 0 ! 0 | 0 ! 0 ! 0 | 0 ! 0 ! 0 ! 0 ! 0 ! 0 ! 0 ! 0 | 0 ! 0 ! 0 ! 0 ! 1 | 0 ! 0 ! 0 | 0 ! 0 ! 0 ! 0 ! 0 | 1 ! 1 ! 1 | 1 ! 1 ! 1 ! 1 ! 1 |
|===!===!===!---!---!---!---!---|---!---!---!---!---!---!---!---|---!---!---!---!---!---!---!---|---!---!---!---!---!---!---!---|---!---!---!---!---!---!---!---|
|   array   |  <-- padding -->  |                         length (1)                            |   u8                          |   value(256)                  |

```


## Map

A map is a collection of key-value tuples. This is encoded in an almost identical fashion to how an Array is encoded, by
specifying the total number of entries (both key & value) that are in the map, then 

