use bytes::Bytes;

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
    err_code: u8,
    err_msg: Bytes,
}

#[derive(Debug, PartialEq)]
pub struct MapDataType(DataType, DataType);
