#![allow(clippy::unusual_byte_groupings)]

use bytes::{BufMut, Bytes};
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) enum DataType {
    Num(Int),
    Bool(BoopBool),
    String(BoopString),
    Error(BoopError),
    Array(BoopArray),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) enum Int {
    Tiny(u8),
    Medium(u32),
    Small(u16),
    Large(u64),
    FloatS(OrderedFloat<f32>),
    FloatL(OrderedFloat<f64>),
}

impl Int {
    pub fn new_u8(v: u8) -> DataType {
        DataType::Num(Int::Tiny(v))
    }
    pub fn new_u16(v: u16) -> DataType {
        DataType::Num(Int::Small(v))
    }
    pub fn new_u32(v: u32) -> DataType {
        DataType::Num(Int::Medium(v))
    }
    pub fn new_u64(v: u64) -> DataType {
        DataType::Num(Int::Large(v))
    }
    pub fn new_f32(v: f32) -> DataType {
        DataType::Num(Int::FloatS(OrderedFloat(v)))
    }
    pub fn new_f64(v: f64) -> DataType {
        DataType::Num(Int::FloatL(OrderedFloat(v)))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct BoopBool(pub bool);

impl BoopBool {
    pub fn new_wrapped(v: bool) -> DataType {
        DataType::Bool(Self(v))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct BoopString(pub Bytes);

impl BoopString {
    pub fn new_wrapped(v: Bytes) -> DataType {
        DataType::String(BoopString(v))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct BoopError {
    pub is_server_err: bool,
    pub err_code: u8,
    pub err_msg: Bytes,
}

impl BoopError {
    pub fn new_unwrapped(is_server_err: bool, err_code: u8, err_msg: Bytes) -> Self {
        Self {
            is_server_err,
            err_code,
            err_msg,
        }
    }

    pub fn wrap(self) -> DataType {
        DataType::Error(self)
    }

    pub fn new_wrapped(is_server_err: bool, err_code: u8, err_msg: Bytes) -> DataType {
        BoopError::wrap(BoopError::new_unwrapped(is_server_err, err_code, err_msg))
    }

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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct BoopArray(pub Vec<DataType>);

impl BoopArray {
    pub fn new_wrapped(data: Vec<DataType>) -> DataType {
        DataType::Array(Self(data))
    }
}
