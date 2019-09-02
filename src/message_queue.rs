
use crate::message::Message;

#[derive(Debug)]
pub(crate) struct MessageQueue {
    messages: Vec<Message>,
}

impl MessageQueue {
    pub fn new() -> MessageQueue {
        MessageQueue{messages: Vec::new()}
    }

    pub fn add(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn pop(&mut self) -> Option<Message> {
        self.messages.pop()
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }
}
