use bytes::{BufMut, Bytes};

#[derive(Debug, PartialEq)]
pub(crate) enum DataType {
    Num(Int),
    Bool(bool),
    String(Bytes),
    Error(Error),
    Array(Box<DataType>),
    Map(Box<MapDataType>),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Int {
    Tiny(u8),
    Medium(u32),
    Small(u16),
    Large(u64),
    FloatS(f32),
    FloatL(f64),
}

#[derive(Debug, PartialEq)]
pub(crate) struct Error {
    pub is_server_err: bool,
    pub err_code: u8,
    // Store error message as byte fields in standard ascii (for now)
    pub err_msg: Bytes,
}

impl Error {
    pub fn encode(&self) -> bytes::BytesMut {
        let mut to_return = bytes::BytesMut::with_capacity(self.err_msg.len() + 5);

        to_return.put_u8(0b_0_0000_110);

        if self.is_server_err {
            to_return.put_u8(1);
        } else {
            to_return.put_u8(0);
        }

        to_return.put_u8(self.err_code);
        to_return.put_u16(self.err_msg.len() as u16);
        to_return.put(self.err_msg.as_ref());

        to_return
    }
}

#[derive(Debug, PartialEq)]
pub struct MapDataType(DataType, DataType);
