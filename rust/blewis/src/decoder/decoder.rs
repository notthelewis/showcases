use bytes::Buf;

use crate::data_type::{DataType, Int};

pub fn handle_decode(buf: &mut bytes::BytesMut) -> anyhow::Result<DataType> {
    // TODO: Better error handling
    match buf.get_u8() {
        // NOTE: All the get_N functions read in BIG ENDIAN order
        0 => Ok(DataType::Num(Int::Tiny(buf.get_u8()))),
        4 => Ok(DataType::Num(Int::Small(buf.get_u16()))),
        8 => Ok(DataType::Num(Int::Medium(buf.get_u32()))),
        16 => Ok(DataType::Num(Int::Large(buf.get_u64()))),
        24 => Ok(DataType::Num(Int::FloatS(buf.get_f32()))),
        28 => Ok(DataType::Num(Int::FloatL(buf.get_f64()))),

        unknown => anyhow::bail!("unkown meta_data byte: {:#b}", unknown),
    }
}

mod test {
    use anyhow::Context;
    use bytes::BufMut as _;

    use super::*;

    #[test]
    fn integer_data_types_decode() {
        let mut buf = bytes::BytesMut::new();

        buf.put_u8(0); // header
        buf.put_u8(0xFF); // u8

        buf.put_u8(0b_0000_0100);
        buf.put_u16(0xFF00);

        buf.put_u8(0b_0000_1000);
        buf.put_u32(0xDEADBEEF);

        buf.put_u8(0b_0001_0000);
        buf.put_u64(0xFEEDFACEDEADBEEF);

        buf.put_u8(0b_0001_1000);
        buf.put_f32(-0.1234);
        buf.put_u8(0b_0001_1000);
        buf.put_f32(0.1234);

        buf.put_u8(0b_0001_1100);
        buf.put_f64(-0.1234);
        buf.put_u8(0b_0001_1100);
        buf.put_f64(0.1234);

        let mut run_test = |expected: DataType, ctxt: &'static str| {
            let received = handle_decode(&mut buf).context(ctxt).unwrap();
            assert_eq!(expected, received);
        };

        run_test(DataType::Num(Int::Tiny(255)), "decode u8");
        run_test(DataType::Num(Int::Small(0xFF00)), "decode u16");
        run_test(DataType::Num(Int::Medium(0xDEADBEEF)), "decode u32");
        run_test(DataType::Num(Int::Large(0xFEEDFACEDEADBEEF)), "decode u64");
        run_test(DataType::Num(Int::FloatS(-0.1234)), "decode negative f32");
        run_test(DataType::Num(Int::FloatS(0.1234)), "decode positive f32");
        run_test(DataType::Num(Int::FloatL(-0.1234)), "decode negative f64");
        run_test(DataType::Num(Int::FloatL(0.1234)), "decode positive f32");
    }
}
