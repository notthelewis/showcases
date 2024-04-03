use anyhow::Ok;
use bytes::Buf;

use crate::{
    data_type::DataType, decoder::decoder::handle_decode, errors::DecodeError, store::Store,
};

/// CmdType is the type of command that is to be parsed/executed.
#[derive(Debug, PartialEq, Eq)]
pub enum CmdType {
    Get,
    GetSet,
    GetDel,
    Set,
}

/// Command is the parsed structure of a Command that manipulates the system in some way.
#[derive(Debug, PartialEq, Eq)]
pub struct Command {
    cmd_type: CmdType,
    key: DataType,
    val: Option<DataType>,
}

impl Command {
    #[inline(always)]
    /// Performs the operations specified by the command
    pub fn execute(self, store: Store) -> Option<DataType> {
        match self.cmd_type {
            CmdType::Get => store.get(&self.key),
            CmdType::GetSet => {
                self.val.as_ref()?;
                Some(store.get_set(&self.key, &self.val.unwrap()))
            }
            CmdType::GetDel => store.get_del(&self.key),
            CmdType::Set => {
                self.val.as_ref()?;
                store.set(&self.key, &self.val.unwrap())
            }
        }
    }
}

#[inline(always)]
pub fn decode_command(buf: &mut bytes::BytesMut) -> anyhow::Result<Command> {
    if buf.is_empty() {
        anyhow::bail!(DecodeError::BufTooShort("command"))
    }

    match buf.get_u8() {
        0x00 => parse_get(buf),
        0x01 => parse_get_set(buf),
        0x02 => parse_get_del(buf),
        0x10 => parse_set(buf),

        byte => anyhow::bail!(DecodeError::UnknownMetaByte(byte)),
    }
}

fn parse_get(buf: &mut bytes::BytesMut) -> anyhow::Result<Command> {
    let key = handle_decode(buf)?;

    Ok(Command {
        cmd_type: CmdType::Get,
        key,
        val: None,
    })
}

fn parse_get_set(buf: &mut bytes::BytesMut) -> anyhow::Result<Command> {
    let key = handle_decode(buf)?;
    let val = handle_decode(buf)?;

    Ok(Command {
        cmd_type: CmdType::GetSet,
        key,
        val: Some(val),
    })
}

fn parse_get_del(buf: &mut bytes::BytesMut) -> anyhow::Result<Command> {
    let key = handle_decode(buf)?;

    Ok(Command {
        cmd_type: CmdType::GetDel,
        key,
        val: None,
    })
}

fn parse_set(buf: &mut bytes::BytesMut) -> anyhow::Result<Command> {
    let key = handle_decode(buf)?;
    let val = handle_decode(buf)?;

    Ok(Command {
        cmd_type: CmdType::Set,
        key,
        val: Some(val),
    })
}

#[cfg(test)]
mod tests {
    use bytes::BufMut;

    use crate::{
        command_interpreter::{parse_get_set, parse_set, CmdType, Command},
        data_type::Int,
    };

    use super::parse_get;

    #[test]
    fn parse_get_valid() {
        let mut buf = bytes::BytesMut::new();
        buf.put_u8(0x00);
        buf.put_u8(0xFF);

        let result = parse_get(&mut buf);
        assert_eq!(
            result.unwrap(),
            Command {
                cmd_type: CmdType::Get,
                key: Int::new_u8(0xFF),
                val: None,
            }
        );
    }

    #[test]
    fn parse_get_set_valid() {
        let mut buf = bytes::BytesMut::new();
        buf.put_u8(0x00);
        buf.put_u8(0xFF);

        buf.put_u8(0x00);
        buf.put_u8(0xFF);

        let result = parse_get_set(&mut buf);
        assert_eq!(
            result.unwrap(),
            Command {
                cmd_type: CmdType::GetSet,
                key: Int::new_u8(0xFF),
                val: Some(Int::new_u8(0xff)),
            }
        );
    }

    #[test]
    fn parse_set_valid() {
        let mut buf = bytes::BytesMut::new();
        buf.put_u8(0x00);
        buf.put_u8(0xFF);

        buf.put_u8(0x00);
        buf.put_u8(0xFF);

        let result = parse_set(&mut buf);
        assert_eq!(
            result.unwrap(),
            Command {
                cmd_type: CmdType::Set,
                key: Int::new_u8(0xFF),
                val: Some(Int::new_u8(0xff)),
            }
        );
    }
}
