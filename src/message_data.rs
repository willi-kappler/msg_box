
#[derive(Debug)]
pub enum MessageData {
    MsgBool(bool),
    MsgU8(u8),
    MsgI8(i8),
    MsgU16(u16),
    MsgI16(i16),
    MsgU32(u32),
    MsgI32(i32),
    MsgU64(u64),
    MsgI64(i64),
    MsgChar(char),
    MsgString(String),
}
