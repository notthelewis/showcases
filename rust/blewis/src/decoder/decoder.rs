use anyhow::{Context, Ok};
use bytes::Buf;

use crate::{
    data_type::{BoopArray, BoopBool, BoopError, BoopString, DataType, Int},
    errors::DecodeError,
};

pub fn handle_decode(buf: &mut bytes::BytesMut) -> anyhow::Result<DataType> {
    // NOTE: Length checks are required before all get calls, as bytes::BufMut will panic if insufficient bytes
    if buf.len() < 1 {
        anyhow::bail!(DecodeError::BufTooShortError("initial meta data byte"))
    }

    match buf.get_u8() {
        // NOTE: All the get_N functions read in BIG ENDIAN order
        0 => {
            if buf.len() < 1 {
                anyhow::bail!(DecodeError::BufTooShortError("u8"))
            }
            Ok(Int::new_u8(buf.get_u8()))
        }
        8 => {
            if buf.len() < 2 {
                anyhow::bail!(DecodeError::BufTooShortError("u16"))
            }
            Ok(Int::new_u16(buf.get_u16()))
        }
        16 => {
            if buf.len() < 4 {
                anyhow::bail!(DecodeError::BufTooShortError("u32"))
            }
            Ok(Int::new_u32(buf.get_u32()))
        }
        32 => {
            if buf.len() < 8 {
                anyhow::bail!(DecodeError::BufTooShortError("u64"))
            }
            Ok(Int::new_u64(buf.get_u64()))
        }
        48 => {
            if buf.len() < 4 {
                anyhow::bail!(DecodeError::BufTooShortError("f32"))
            }
            Ok(Int::new_f32(buf.get_f32()))
        }
        56 => {
            if buf.len() < 8 {
                anyhow::bail!(DecodeError::BufTooShortError("f64"))
            }
            Ok(Int::new_f64(buf.get_f64()))
        }

        // NOTE: Boolean values don't require any more data. It's just the 1 byte we already read
        132 => Ok(BoopBool::new(true)),
        4 => Ok(BoopBool::new(false)),

        // Strings are length prepended byte arrays. We use the `copy_to_bytes` function to
        // leverage the Bytes package's shallow copy mechanism, as opposed to making a full copy.
        2 => {
            if buf.len() < 2 {
                anyhow::bail!(DecodeError::BufTooShortError("string header"))
            }
            let len = buf.get_u16() as usize;
            if buf.len() < len {
                anyhow::bail!(DecodeError::BufTooShortError("string contents"))
            }
            Ok(BoopString::new(buf.copy_to_bytes(len)))
        }

        // Error
        6 => {
            if buf.len() < 4 {
                anyhow::bail!(DecodeError::BufTooShortError("error header"))
            }
            let is_server_err = buf.get_u8() != 0;
            let err_code = buf.get_u8();
            let err_len = buf.get_u16() as usize;
            if buf.len() < err_len {
                anyhow::bail!(DecodeError::BufTooShortError("error value"))
            }
            let err_msg = buf.copy_to_bytes(err_len);
            Ok(BoopError::new(is_server_err, err_code, err_msg))
        }

        // Array
        3 => {
            if buf.len() < 2 {
                anyhow::bail!(DecodeError::BufTooShortError("array header"))
            }
            let element_length = buf.get_u16();
            let mut index = 0;
            let mut data: Vec<DataType> = Vec::with_capacity(element_length as usize);

            // TODO: Add recursion depth check
            while index < element_length {
                let result = handle_decode(buf);
                if result.is_err() {
                    return result
                        .with_context(|| format!("array decode failed on element: {}", index));
                }
                data.push(result.unwrap());
                index += 1;
            }

            Ok(BoopArray::new(data))
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
        println!("{:?}", received);
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

        _run_test(&mut buf, BoopBool::new(true), "decode a TRUE");
        _run_test(&mut buf, BoopBool::new(false), "decode a FALSE");
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
            BoopString::new(Bytes::from_static(to_encode)),
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

        let err_to_encode = crate::data_type::BoopError {
            is_server_err: false,
            err_code: 0x00,
            err_msg: bytes::Bytes::from_static(b"some message"),
        };
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
            BoopBool::new(true),
            BoopBool::new(false),
            BoopString::new(Bytes::from_static(string_to_encode)),
            DataType::Error(err_to_encode),
        ];

        _run_test(
            &mut buf,
            BoopArray::new(expected),
            "array decode",
        );
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

        let expected = BoopArray::new(vec![
            BoopArray::new(vec![
                BoopBool::new(true),
                BoopBool::new(false),
            ]),
            BoopBool::new(true),
        ]);

        _run_test(&mut buf, expected, "array decode nested array");
    }
}
