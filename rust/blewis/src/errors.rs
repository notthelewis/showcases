use bytes::Bytes;
use thiserror::Error;

use crate::data_type::{BoopError, DataType};

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Unable to decode {0} from buffer as buffer's length is too small")]
    BufTooShort(&'static str),

    #[error("Unable to decode buffer due to unknown meta byte: {0}")]
    UnknownMetaByte(u8),
}

/// GETSET_KEY_NO_EXIST is emitted by the GetSet command.
pub const GETSET_KEY_NO_EXIST: DataType = DataType::Error(BoopError {
    is_server_err: true,
    err_code: 0x10,
    err_msg: Bytes::from_static(b"no_exist"),
});
