use anyhow::{Context, Ok};
use bytes::Buf;
use ordered_float::OrderedFloat;

use crate::data_type::{DataType, Int, MapEntry};

pub fn handle_decode(buf: &mut bytes::BytesMut) -> anyhow::Result<DataType> {
    // TODO: Better error handling
    match buf.get_u8() {
        // NOTE: All the get_N functions read in BIG ENDIAN order
        0 => Ok(DataType::Num(Int::Tiny(buf.get_u8()))),
        8 => Ok(DataType::Num(Int::Small(buf.get_u16()))),
        16 => Ok(DataType::Num(Int::Medium(buf.get_u32()))),
        32 => Ok(DataType::Num(Int::Large(buf.get_u64()))),
        48 => Ok(DataType::Num(Int::FloatS(OrderedFloat(buf.get_f32())))),
        56 => Ok(DataType::Num(Int::FloatL(OrderedFloat(buf.get_f64())))),

        // Boolean values don't require any more data. It's just the 1 byte
        132 => Ok(DataType::Bool(true)),
        4 => Ok(DataType::Bool(false)),

        // Strings are length prepended byte arrays. We use the `copy_to_bytes` function to
        // leverage the Bytes package's shallow copy mechanism, as opposed to making a full copy.
        2 => {
            let len = buf.get_u16() as usize;
            Ok(DataType::String(buf.copy_to_bytes(len)))
        }

        6 => {
            let is_server_err = buf.get_u8() != 0;
            let err_code = buf.get_u8();
            let err_len = buf.get_u16() as usize;
            let err_msg = buf.copy_to_bytes(err_len);
            Ok(DataType::Error(crate::data_type::Error {
                is_server_err,
                err_code,
                err_msg,
            }))
        }

        // Array
        3 => {
            let element_length = buf.get_u16();
            let mut index = 0;
            let mut data: Vec<DataType> = Vec::with_capacity(element_length as usize);

            while index < element_length {
                let result = handle_decode(buf);
                if result.is_err() {
                    return result
                        .with_context(|| format!("array decode failed on element: {}", index));

                }
                data.push(result.unwrap());
                index += 1;
            }

            Ok(DataType::Array(data))
        }

        // Map
        7 => {
            let element_length = buf.get_u16();
            let mut index = 0;
            let mut arr: Vec<MapEntry> = Vec::with_capacity(element_length as usize);
            
            while index < element_length {
                let key = handle_decode(buf);
                if key.is_err() {
                    return key
                        .with_context(|| format!("map decode failed on KEY at element: {}", index));
                }
                let val = handle_decode(buf);
                if val.is_err() {
                    return val
                        .with_context(|| format!("map decode failed on VALUE at element: {}", index));
                }
                let key = key.unwrap();
                let val = val.unwrap();
                arr.push(MapEntry{
                    key,
                    val
                });
                index += 1;
            }

            Ok(DataType::Map(arr))
        }

        unknown => anyhow::bail!("unknown meta_data byte: {:#b}", unknown),
    }
}

mod test {
    use std::iter::Map;

    use anyhow::Context;
    use bytes::BufMut;

    use super::*;

    fn run_test(buf: &mut bytes::BytesMut, expected: DataType, ctxt: &'static str) {
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

        run_test(&mut buf, DataType::Num(Int::Tiny(255)), "decode u8");
        run_test(&mut buf, DataType::Num(Int::Small(0xFF00)), "decode u16");
        run_test(
            &mut buf,
            DataType::Num(Int::Medium(0xDEADBEEF)),
            "decode u32",
        );
        run_test(
            &mut buf,
            DataType::Num(Int::Large(0xFEEDFACEDEADBEEF)),
            "decode u64",
        );
        run_test(
            &mut buf,
            DataType::Num(Int::FloatS(OrderedFloat(-0.1234))),
            "decode negative f32",
        );
        run_test(
            &mut buf,
            DataType::Num(Int::FloatS(OrderedFloat(0.1234))),
            "decode positive f32",
        );
        run_test(
            &mut buf,
            DataType::Num(Int::FloatL(OrderedFloat(-0.1234))),
            "decode negative f64",
        );
        run_test(
            &mut buf,
            DataType::Num(Int::FloatL(OrderedFloat(0.1234))),
            "decode positive f64",
        );
    }

    #[test]
    fn bool_decode() {
        let mut buf = bytes::BytesMut::new();
        buf.put_u8(0b_1_0000_100); // true
        buf.put_u8(0b_0_0000_100); // false

        run_test(&mut buf, DataType::Bool(true), "decode a TRUE");
        run_test(&mut buf, DataType::Bool(false), "decode a FALSE");
    }

    #[test]
    fn string_decode() {
        let mut buf = bytes::BytesMut::new();
        let to_encode = b"multiple\r\nlines\r\nsupported\0null bytes too";

        buf.put_u8(0b_00000_010); // string type
        buf.put_u16(to_encode.len() as u16); // length
        buf.put_slice(to_encode);

        run_test(
            &mut buf,
            DataType::String(bytes::Bytes::from_static(to_encode)),
            "decode a string",
        );
    }

    #[test]
    fn error_decode() {
        let err = crate::data_type::Error {
            is_server_err: false,
            err_code: 0xFF,
            err_msg: bytes::Bytes::from_static(b"err"),
        };
        let mut buf = err.encode();
        run_test(&mut buf, DataType::Error(err), "decode an error");
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

        let err_to_encode = crate::data_type::Error {
            is_server_err: false,
            err_code: 0x00,
            err_msg: bytes::Bytes::from_static(b"some message"),
        };
        buf.put(err_to_encode.encode());

        let expected: Vec<DataType> = vec![
            DataType::Num(Int::Tiny(255)),
            DataType::Num(Int::Small(0xFF00)),
            DataType::Num(Int::Medium(0xDEADBEEF)),
            DataType::Num(Int::Large(0xFEEDFACEDEADBEEF)),
            DataType::Num(Int::FloatS(OrderedFloat(-0.1234))),
            DataType::Num(Int::FloatS(OrderedFloat(0.1234))),
            DataType::Num(Int::FloatL(OrderedFloat(-0.1234))),
            DataType::Num(Int::FloatL(OrderedFloat(0.1234))),
            DataType::Bool(true),
            DataType::Bool(false),
            DataType::String(bytes::Bytes::from_static(string_to_encode)),
            DataType::Error(err_to_encode),
        ];

        run_test(&mut buf, DataType::Array(expected), "array decode");
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

        let expected = DataType::Array(vec![
            DataType::Array(vec![DataType::Bool(true), DataType::Bool(false)]),
            DataType::Bool(true),
        ]);

        run_test(&mut buf, expected, "array decode nested array");
    }

    #[test]
    fn map_decode() {
        let mut buf = bytes::BytesMut::new();

        buf.put_u8(0b_0_0000_111); // HEADER = MAP
        buf.put_u16(11); // MAP ENTRY LEN

        // TYPE = TINY
        buf.put_u8(0);
        buf.put_u8(0xFF);
        buf.put_u8(0);
        buf.put_u8(0xFF);

        // TYPE = SMALL
        buf.put_u8(0b_00_001_000);
        buf.put_u16(0xFF00);
        buf.put_u8(0b_00_001_000);
        buf.put_u16(0xFF00);

        // TYPE = MEDIUM
        buf.put_u8(0b_00_010_000);
        buf.put_u32(0xDEADBEEF);
        buf.put_u8(0b_00_010_000);
        buf.put_u32(0xDEADBEEF);

        // TYPE = LARGE
        buf.put_u8(0b_00_100_000);
        buf.put_u64(0xFEEDFACEDEADBEEF);
        buf.put_u8(0b_00_100_000);
        buf.put_u64(0xFEEDFACEDEADBEEF);

        // TYPE = FLOATING_S+
        buf.put_u8(0b_00_110_000);
        buf.put_f32(0.1234);
        buf.put_u8(0b_00_110_000);
        buf.put_f32(0.1234);
        // TYPE = FLOATING_S-
        buf.put_u8(0b_00_110_000);
        buf.put_f32(-0.1234);
        buf.put_u8(0b_00_110_000);
        buf.put_f32(-0.1234);
        
        // TYPE = FLOATING_L+
        buf.put_u8(0b_00_111_000);
        buf.put_f64(0.1234);
        buf.put_u8(0b_00_111_000);
        buf.put_f64(0.1234);
        // TYPE = FLOATING_L-
        buf.put_u8(0b_00_111_000);
        buf.put_f64(-0.1234);
        buf.put_u8(0b_00_111_000);
        buf.put_f64(-0.1234);

        // TYPE = BOOL 
        buf.put_u8(0b_1_0000_100); // True
        buf.put_u8(0b_0_0000_100); // False

        // TYPE = STRING
        let string_to_encode = b"multiple\r\nlines\r\nsupported\0null bytes too";
        buf.put_u8(0b_00000_010);
        buf.put_u16(string_to_encode.len() as u16);
        buf.put_slice(string_to_encode);
        buf.put_u8(0b_00000_010);
        buf.put_u16(string_to_encode.len() as u16);
        buf.put_slice(string_to_encode);

        let err_to_encode = crate::data_type::Error {
            is_server_err: false,
            err_code: 0x00,
            err_msg: bytes::Bytes::from_static(b"some message"),
        };

        buf.put(err_to_encode.encode());
        buf.put(err_to_encode.encode());

        let expected :Vec<MapEntry> = vec![
            MapEntry{
                key: DataType::Num(Int::Tiny(0xFF)),
                val: DataType::Num(Int::Tiny(0xFF))
            },
            MapEntry{
                key: DataType::Num(Int::Small(0xFF00)),
                val: DataType::Num(Int::Small(0xFF00))
            },
            MapEntry{
                key: DataType::Num(Int::Medium(0xDEADBEEF)),
                val: DataType::Num(Int::Medium(0xDEADBEEF)),
            },
            MapEntry{
                key: DataType::Num(Int::Large(0xFEEDFACEDEADBEEF)),
                val: DataType::Num(Int::Large(0xFEEDFACEDEADBEEF))
            },
            MapEntry{
                key: DataType::Num(Int::FloatS(OrderedFloat(0.1234))),
                val: DataType::Num(Int::FloatS(OrderedFloat(0.1234))) 
            },
            MapEntry{
                key: DataType::Num(Int::FloatS(OrderedFloat(-0.1234))),
                val: DataType::Num(Int::FloatS(OrderedFloat(-0.1234))) 
            },
            MapEntry{
                key: DataType::Num(Int::FloatL(OrderedFloat(0.1234))),
                val: DataType::Num(Int::FloatL(OrderedFloat(0.1234))) 
            },
            MapEntry{
                key: DataType::Num(Int::FloatL(OrderedFloat(-0.1234))),
                val: DataType::Num(Int::FloatL(OrderedFloat(-0.1234))) 
            },
            MapEntry{
                key: DataType::Bool(true),
                val: DataType::Bool(false), 
            },
            MapEntry{
                key: DataType::String(bytes::Bytes::from_static(string_to_encode)),
                val: DataType::String(bytes::Bytes::from_static(string_to_encode)), 
            },
            MapEntry{
                key: DataType::Error(err_to_encode.clone()),
                val: DataType::Error(err_to_encode.clone()),
            },
        ];

        run_test(&mut buf, DataType::Map(expected), "map decode");
    }
}
