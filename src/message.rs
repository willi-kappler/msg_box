
use crate::message_data::MessageData;

#[derive(Debug)]
pub struct Message {
    pub sender: String,
    pub receiver: String,
    pub data: Vec<MessageData>,
}
