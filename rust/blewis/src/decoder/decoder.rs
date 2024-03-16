use bytes::Buf;

use crate::data_type::{DataType, Int};

pub fn handle_decode(buf: &mut bytes::BytesMut) -> anyhow::Result<DataType> {
    // TODO: Better error handling
    match buf.get_u8() {
        // NOTE: All the get_N functions read in BIG ENDIAN order
        0 => Ok(DataType::Num(Int::Tiny(buf.get_u8()))),
        8 => Ok(DataType::Num(Int::Small(buf.get_u16()))),
        16 => Ok(DataType::Num(Int::Medium(buf.get_u32()))),
        32 => Ok(DataType::Num(Int::Large(buf.get_u64()))),
        48 => Ok(DataType::Num(Int::FloatS(buf.get_f32()))),
        56 => Ok(DataType::Num(Int::FloatL(buf.get_f64()))),

        132 => Ok(DataType::Bool(true)),
        4 => Ok(DataType::Bool(false)),

        unknown => anyhow::bail!("unknown meta_data byte: {:#b}", unknown),
    }
}

mod test {
    use anyhow::Context;
    use bytes::BufMut;

    use super::*;

    fn run_test(buf: &mut bytes::BytesMut, expected: DataType, ctxt: &'static str) {
        let received = handle_decode(buf).context(ctxt).unwrap();
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
        run_test(&mut buf, DataType::Num(Int::Medium(0xDEADBEEF)), "decode u32");
        run_test(&mut buf, DataType::Num(Int::Large(0xFEEDFACEDEADBEEF)), "decode u64");
        run_test(&mut buf, DataType::Num(Int::FloatS(-0.1234)), "decode negative f32");
        run_test(&mut buf, DataType::Num(Int::FloatS(0.1234)), "decode positive f32");
        run_test(&mut buf, DataType::Num(Int::FloatL(-0.1234)), "decode negative f64");
        run_test(&mut buf, DataType::Num(Int::FloatL(0.1234)), "decode positive f32");
    }

    #[test]
    fn bool_decode() {
        let mut buf = bytes::BytesMut::new();
        buf.put_u8(0b_1_0000_100);
        buf.put_u8(0b_0_0000_100);

        run_test(&mut buf, DataType::Bool(true), "should decode a TRUE");
        run_test(&mut buf, DataType::Bool(false), "should decode a FALSE");
    }
}
