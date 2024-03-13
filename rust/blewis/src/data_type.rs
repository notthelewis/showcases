use bytes::Bytes;

pub(crate) enum DataType {
    Integer(IntegerDataType),
    Bool(bool),
    String(Bytes),
    Error(ErrorDataType),
    Array(Box<DataType>),
    Map(Box<(DataType, DataType)>),
}

pub(crate) enum IntegerDataType {
    Tiny(u8),
    Medium(u32),
    Large(u64),
    FloatingSmall(f32),
    FloatingLarge(f64)
}

pub(crate) struct ErrorDataType {
    err_code: u8,
    err_msg: Bytes
}
