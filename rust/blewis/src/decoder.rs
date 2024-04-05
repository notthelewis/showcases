use crate::{
    data_type::{BoopArray, BoopBool, BoopError, BoopString, DataType, Int},
    errors::DecodeError,
};
use anyhow::{Context, Ok};
use bytes::{Buf, BufMut, BytesMut};

/// Check the buffer's length contains at least `len` bytes and if it doesn't, put the following
/// `meta_bytes` into the buffer and return a DecodeError::BufTooShort, with the given `buf_msg`
#[inline(always)]
fn check_len(
    buf: &mut BytesMut,
    len: usize,
    meta_bytes: &[u8],
    buf_msg: &'static str,
) -> anyhow::Result<()> {
    if buf.len() < len {
        if !meta_bytes.is_empty() {
            // TODO: Potential panic case. Exponential read buffer growth
            buf.put(meta_bytes);
        }
        anyhow::bail!(DecodeError::BufTooShort(buf_msg))
    }
    anyhow::Ok(())
}

pub fn handle_decode(buf: &mut BytesMut) -> anyhow::Result<DataType> {
    // NOTE: Length checks are required before all get calls, as bytes::BufMut will panic if insufficient bytes
    check_len(buf, 1, &[], "meta data byte")?;

    let meta_byte = buf.get_u8();

    match meta_byte {
        // NOTE: All the get_N functions read in BIG ENDIAN order
        0 => {
            check_len(buf, 1, &[meta_byte], "u8")?;
            Ok(Int::new_u8(buf.get_u8()))
        }
        8 => {
            check_len(buf, 2, &[meta_byte], "u16")?;
            Ok(Int::new_u16(buf.get_u16()))
        }
        16 => {
            check_len(buf, 4, &[meta_byte], "u32")?;
            Ok(Int::new_u32(buf.get_u32()))
        }
        32 => {
            check_len(buf, 8, &[meta_byte], "u64")?;
            Ok(Int::new_u64(buf.get_u64()))
        }
        48 => {
            check_len(buf, 4, &[meta_byte], "f32")?;
            Ok(Int::new_f32(buf.get_f32()))
        }
        56 => {
            check_len(buf, 8, &[meta_byte], "f64")?;
            Ok(Int::new_f64(buf.get_f64()))
        }

        // NOTE: Boolean values don't require any more data. It's just the 1 byte we already read
        132 => Ok(BoopBool::new_wrapped(true)),
        4 => Ok(BoopBool::new_wrapped(false)),

        // Strings are length prepended byte arrays. We use the `copy_to_bytes` function to
        // leverage the Bytes package's shallow copy mechanism, as opposed to making a full copy.
        2 => {
            check_len(buf, 2, &[2], "string header")?;

            let str_len = buf.get_u16();

            check_len(
                buf,
                str_len as usize,
                &[&[2u8], &str_len.to_be_bytes()[..]].concat(),
                "string contents",
            )?;

            Ok(BoopString::new_wrapped(buf.copy_to_bytes(str_len as usize)))
        }

        // Error
        6 => {
            check_len(buf, 4, &[6], "error header")?;
            let is_server_err = buf.get_u8();
            let err_code = buf.get_u8();
            let err_len = buf.get_u16() as usize;

            check_len(
                buf,
                err_len,
                &[&[is_server_err, err_code], &err_len.to_be_bytes()[..]].concat(),
                "error value",
            )?;
            let err_msg = buf.copy_to_bytes(err_len);

            Ok(BoopError::new_wrapped(
                is_server_err != 0,
                err_code,
                err_msg,
            ))
        }

        // Array
        3 => {
            check_len(buf, 2, &[3], "array header")?;
            let element_length = buf.get_u16();
            let mut index = 0;
            let mut data: Vec<DataType> = Vec::with_capacity(element_length as usize);

            // TODO: Work out how to do this better
            let pre_array_start = bytes::BytesMut::from(&buf[..]);

            // TODO: Add recursion depth check
            while index < element_length {
                let result = handle_decode(buf);
                if result.is_err() {
                    *buf = pre_array_start;
                    return result
                        .with_context(|| format!("array decode failed on element: {}", index));
                }
                data.push(result.unwrap());
                index += 1;
            }

            Ok(BoopArray::new_wrapped(data))
        }

        unknown => Err(anyhow::anyhow!(DecodeError::UnknownMetaByte(unknown))),
    }
}

mod test {
    #![allow(unused_imports)]

    use super::*;
    use anyhow::Context;
    use bytes::{BufMut, Bytes};

    fn _run_test(buf: &mut bytes::BytesMut, expected: DataType, ctxt: &'static str) {
        let received = handle_decode(buf).context(ctxt).unwrap();
        // println!("{:?}", received);
        assert_eq!(expected, received);
    }

    #[test]
    fn integer_data_types_decode() {
        let mut buf = bytes::BytesMut::new();

        buf.put_u8(0); // tiny
        buf.put_u8(0xFF);

        buf.put_u8(0b_00_001_000); // small
        buf.put_u16(0xFF00);

        buf.put_u8(0b_00_010_000); // medium
        buf.put_u32(0xDEADBEEF);

        buf.put_u8(0b_00_100_000); // large
        buf.put_u64(0xFEEDFACEDEADBEEF);

        buf.put_u8(0b_00_110_000); // floatingS
        buf.put_f32(-0.1234);
        buf.put_u8(0b_00_110_000);
        buf.put_f32(0.1234);

        buf.put_u8(0b_00_111_000); // floatingL
        buf.put_f64(-0.1234);
        buf.put_u8(0b_00_111_000);
        buf.put_f64(0.1234);

        _run_test(&mut buf, Int::new_u8(255), "decode u8");
        _run_test(&mut buf, Int::new_u16(0xFF00), "decode u16");
        _run_test(&mut buf, Int::new_u32(0xDEADBEEF), "decode u32");
        _run_test(&mut buf, Int::new_u64(0xFEEDFACEDEADBEEF), "decode u64");
        _run_test(&mut buf, Int::new_f32(-0.1234), "decode negative f32");
        _run_test(&mut buf, Int::new_f32(0.1234_f32), "decode positive f32");
        _run_test(&mut buf, Int::new_f64(-0.1234_f64), "decode negative f64");
        _run_test(&mut buf, Int::new_f64(0.1234), "decode positive f64");
    }

    #[test]
    fn bool_decode() {
        let mut buf = bytes::BytesMut::new();
        buf.put_u8(0b_1_0000_100); // true
        buf.put_u8(0b_0_0000_100); // false

        _run_test(&mut buf, BoopBool::new_wrapped(true), "decode a TRUE");
        _run_test(&mut buf, BoopBool::new_wrapped(false), "decode a FALSE");
    }

    #[test]
    fn string_decode() {
        let mut buf = bytes::BytesMut::new();
        let to_encode = b"multiple\r\nlines\r\nsupported\0null bytes too";

        buf.put_u8(0b_00000_010); // string type
        buf.put_u16(to_encode.len() as u16); // length
        buf.put_slice(to_encode);

        _run_test(
            &mut buf,
            BoopString::new_wrapped(Bytes::from_static(to_encode)),
            "decode a string",
        );
    }

    #[test]
    fn error_decode() {
        let err = crate::data_type::BoopError {
            is_server_err: false,
            err_code: 0xFF,
            err_msg: bytes::Bytes::from_static(b"err"),
        };
        let mut buf = err.encode();
        _run_test(&mut buf, DataType::Error(err), "decode an error");
    }

    #[test]
    fn array_decode_simple_types() {
        let mut buf = bytes::BytesMut::new();

        buf.put_u8(0b_0_0000_011); // HEADER = ARRAY
        buf.put_u16(12); // ARRAY ELEMENT LEN

        // TINY
        buf.put_u8(0);
        buf.put_u8(0xFF);
        // SMALL
        buf.put_u8(0b_00_001_000);
        buf.put_u16(0xFF00);
        // MEDIUM
        buf.put_u8(0b_00_010_000);
        buf.put_u32(0xDEADBEEF);
        // LARGE
        buf.put_u8(0b_00_100_000);
        buf.put_u64(0xFEEDFACEDEADBEEF);
        // FLOATING_S
        buf.put_u8(0b_00_110_000);
        buf.put_f32(-0.1234);
        buf.put_u8(0b_00_110_000);
        buf.put_f32(0.1234);
        // FLOATING_L
        buf.put_u8(0b_00_111_000);
        buf.put_f64(-0.1234);
        buf.put_u8(0b_00_111_000);
        buf.put_f64(0.1234);

        // BOOL=TRUE
        buf.put_u8(0b_1_0000_100);
        // BOOL=FALSE
        buf.put_u8(0b_0_0000_100);

        let string_to_encode = b"multiple\r\nlines\r\nsupported\0null bytes too";
        buf.put_u8(0b_00000_010);
        buf.put_u16(string_to_encode.len() as u16);
        buf.put_slice(string_to_encode);

        let err_to_encode =
            BoopError::new_unwrapped(false, 0x00, Bytes::from_static(b"some message"));
        buf.put(err_to_encode.encode());

        let expected: Vec<DataType> = vec![
            Int::new_u8(255_u8),
            Int::new_u16(0xFF00_u16),
            Int::new_u32(0xDEADBEEF_u32),
            Int::new_u64(0xFEEDFACEDEADBEEF_u64),
            Int::new_f32(-0.1234_f32),
            Int::new_f32(0.1234_f32),
            Int::new_f64(-0.1234_f64),
            Int::new_f64(0.1234_f64),
            BoopBool::new_wrapped(true),
            BoopBool::new_wrapped(false),
            BoopString::new_wrapped(Bytes::from_static(string_to_encode)),
            BoopError::wrap(err_to_encode),
        ];

        _run_test(&mut buf, BoopArray::new_wrapped(expected), "array decode");
    }

    #[test]
    fn array_decode_nested_array() {
        let mut buf = bytes::BytesMut::new();

        buf.put_u8(0b_0_0000_011); // HEADER = ARRAY
        buf.put_u16(2); // ARRAY ELEMENT LEN
        {
            {
                buf.put_u8(0b_0_0000_011); // HEADER = ARRAY
                buf.put_u16(2); // ARRAY ELEMENT LEN
                {
                    // BOOL=TRUE
                    buf.put_u8(0b_1_0000_100);
                }
                {
                    // BOOL=FALSE
                    buf.put_u8(0b_0_0000_100);
                }
            }
            {
                // BOOL=TRUE
                buf.put_u8(0b_1_0000_100);
            }
        }

        let expected = BoopArray::new_wrapped(vec![
            BoopArray::new_wrapped(vec![
                BoopBool::new_wrapped(true),
                BoopBool::new_wrapped(false),
            ]),
            BoopBool::new_wrapped(true),
        ]);

        _run_test(&mut buf, expected, "array decode nested array");
    }

    #[test]
    fn check_len_u16_recomposition() {
        let mut buf = bytes::BytesMut::new();
        buf.put_u16(255);

        let str_len = buf.get_u16();
        assert_eq!(str_len, 255);

        assert!(check_len(
            &mut buf,
            str_len as usize,
            &[&[2u8], &str_len.to_be_bytes()[..]].concat(),
            "string contents",
        )
        .is_err());

        // Add 3 because check_len should put the meta byte and the two length bytes back
        assert_eq!(buf.len(), 3);

        // Test the right things are put back into the buffer
        assert_eq!(buf.get_u8(), 2);
        assert_eq!(buf.get_u16(), 255);

        for i in 0..str_len {
            buf.put_u8(i as u8);
        }

        assert_eq!(buf.len(), 255);

        assert!(check_len(
            &mut buf,
            str_len as usize,
            &[&[2u8], &str_len.to_be_bytes()[..]].concat(),
            "string contents",
        )
        .is_ok());
    }

    #[test]
    fn missing_meta_byte() {
        let mut buf = BytesMut::new();
        assert!(handle_decode(&mut buf).is_err());
        let err = handle_decode(&mut buf).unwrap_err();
        assert_eq!(
            err.to_string(),
            DecodeError::BufTooShort("meta data byte").to_string()
        );
    }


    #[test]
    /// The insufficient_bytes_for_N tests each validate that the length validation function works,
    /// and that it leaves any unprocessed bytes on the FIFO stack to be popped off later. 
    fn insufficient_bytes_for_uint8() {
        let mut buf = BytesMut::new();
        buf.put_u8(0x00); // u8
        let err = handle_decode(&mut buf);
        assert!(err.is_err());
        assert_eq!(
            err.unwrap_err().to_string(),
            DecodeError::BufTooShort("u8").to_string()
        );
        assert!(buf.get_u8() == 0x00);
    }

    #[test]
    fn insufficient_bytes_for_uint16() {
        let mut buf = BytesMut::new();
        buf.put_u8(0x8); // u16
        buf.put_u8(0x00);
        let err = handle_decode(&mut buf);
        assert!(err.is_err());
        assert_eq!(
            err.unwrap_err().to_string(),
            DecodeError::BufTooShort("u16").to_string()
        );
        assert!(buf.get_u8() == 0x00);
        assert!(buf.get_u8() == 0x8);
    }

    #[test]
    fn insufficient_bytes_for_uint32() {
        let mut buf = BytesMut::new();
        buf.put_u8(0x10); // u32
        buf.put_u8(0x00);
        buf.put_u8(0x01);
        buf.put_u8(0x02);
        let err = handle_decode(&mut buf);
        assert!(err.is_err());
        assert_eq!(
            err.unwrap_err().to_string(),
            DecodeError::BufTooShort("u32").to_string()
        );
        assert!(buf.get_u8() == 0x00);
        assert!(buf.get_u8() == 0x01);
        assert!(buf.get_u8() == 0x02);
        assert!(buf.get_u8() == 0x10);
    }

    #[test]
    fn insufficient_bytes_for_uint64() {
        let mut buf = BytesMut::new();
        buf.put_u8(0x20); // u64
        buf.put_u8(0x00);
        buf.put_u8(0x01);
        buf.put_u8(0x02);
        buf.put_u8(0x03);
        buf.put_u8(0x04);
        buf.put_u8(0x05);
        buf.put_u8(0x06);

        let err = handle_decode(&mut buf);
        assert!(err.is_err());
        assert_eq!(
            err.unwrap_err().to_string(),
            DecodeError::BufTooShort("u64").to_string()
        );

        assert_eq!(buf.get_u8(), 0x00);
        assert_eq!(buf.get_u8(), 0x01);
        assert_eq!(buf.get_u8(), 0x02);
        assert_eq!(buf.get_u8(), 0x03);
        assert_eq!(buf.get_u8(), 0x04);
        assert_eq!(buf.get_u8(), 0x05);
        assert_eq!(buf.get_u8(), 0x06);
        assert_eq!(buf.get_u8(), 0x20);
    }

    #[test]
    fn insufficient_bytes_for_float32() {
        let mut buf = BytesMut::new();
        buf.put_u8(0x30); // u32
        buf.put_u8(0x00);
        buf.put_u8(0x01);
        buf.put_u8(0x02);
        let err = handle_decode(&mut buf);
        assert!(err.is_err());
        assert_eq!(
            err.unwrap_err().to_string(),
            DecodeError::BufTooShort("f32").to_string()
        );
        assert!(buf.get_u8() == 0x00);
        assert!(buf.get_u8() == 0x01);
        assert!(buf.get_u8() == 0x02);
        assert!(buf.get_u8() == 0x30);
    }

    #[test]
    fn insufficient_bytes_for_float64() {
        let mut buf = BytesMut::new();
        buf.put_u8(0x38); // f64
        buf.put_u8(0x00);
        buf.put_u8(0x01);
        buf.put_u8(0x02);
        buf.put_u8(0x03);
        buf.put_u8(0x04);
        buf.put_u8(0x05);
        buf.put_u8(0x06);

        let err = handle_decode(&mut buf);
        assert!(err.is_err());
        assert_eq!(
            err.unwrap_err().to_string(),
            DecodeError::BufTooShort("f64").to_string()
        );

        assert_eq!(buf.get_u8(), 0x00);
        assert_eq!(buf.get_u8(), 0x01);
        assert_eq!(buf.get_u8(), 0x02);
        assert_eq!(buf.get_u8(), 0x03);
        assert_eq!(buf.get_u8(), 0x04);
        assert_eq!(buf.get_u8(), 0x05);
        assert_eq!(buf.get_u8(), 0x06);
        assert_eq!(buf.get_u8(), 0x38);
    }
}
