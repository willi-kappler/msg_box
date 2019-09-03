
use crate::message::Message;

#[derive(Debug)]
pub(crate) struct MessageQueue {
    messages: Vec<Message>,
}

impl MessageQueue {
    pub(crate) fn new() -> MessageQueue {
        MessageQueue{messages: Vec::new()}
    }

    pub(crate) fn add(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub(crate) fn pop(&mut self) -> Option<Message> {
        self.messages.pop()
    }

    pub(crate) fn len(&self) -> usize {
        self.messages.len()
    }
}
