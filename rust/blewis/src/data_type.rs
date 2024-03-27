use bytes::{BufMut, Bytes};
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum DataType {
    Num(Int),
    Bool(bool),
    String(Bytes),
    Error(Error),
    Array(Vec<DataType>),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum Int {
    Tiny(u8),
    Medium(u32),
    Small(u16),
    Large(u64),
    FloatS(OrderedFloat<f32>),
    FloatL(OrderedFloat<f64>),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Error {
    pub is_server_err: bool,
    pub err_code: u8,
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
