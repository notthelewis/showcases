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
