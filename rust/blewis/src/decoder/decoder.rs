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
        24 => Ok(DataType::Num(Int::FloatSmall(buf.get_f32()))),
        28 => Ok(DataType::Num(Int::FloatLarge(buf.get_f64()))),

        unknown => anyhow::bail!("unkown meta_data byte: {:#b}", unknown)
    }
}


mod test {
    use bytes::BufMut as _;
    use anyhow::Context;

    use super::*;

    #[test]
    fn integer_data_types_decode() {
        let mut buf = bytes::BytesMut::new();

        buf.put_u8(0);
        buf.put_u8(0xFF);

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

        let expected = DataType::Num(Int::Tiny(255));
        let received = handle_decode(&mut buf).context("decode u8").unwrap();
        assert_eq!(expected, received);

        let expected = DataType::Num(Int::Small(0xFF00));
        let received = handle_decode(&mut buf).context("decode u16").unwrap();
        assert_eq!(expected, received);

        let expected = DataType::Num(Int::Medium(0xDEADBEEF));
        let received = handle_decode(&mut buf).context("decode u32").unwrap();
        assert_eq!(expected, received);

        let expected = DataType::Num(Int::Large(0xFEEDFACEDEADBEEF));
        let received = handle_decode(&mut buf).context("decode u64").unwrap();
        assert_eq!(expected, received);

        let expected = DataType::Num(Int::FloatSmall(-0.1234));
        let received = handle_decode(&mut buf).context("decode f32 negative number").unwrap();
        assert_eq!(expected, received);

        let expected = DataType::Num(Int::FloatSmall(0.1234));
        let received = handle_decode(&mut buf).context("decode f32 positive number").unwrap();
        assert_eq!(expected, received);

        let expected = DataType::Num(Int::FloatLarge(-0.1234));
        let received = handle_decode(&mut buf).context("decode f64 negative number").unwrap();
        assert_eq!(expected, received);

        let expected = DataType::Num(Int::FloatLarge(0.1234));
        let received = handle_decode(&mut buf).context("decode f64 positive number").unwrap();
        assert_eq!(expected, received);
    }
}
