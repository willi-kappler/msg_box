// Rust modules
use std::sync::{Arc, Mutex};

// Internal modules
use crate::message_scheduler::{MessageScheduler, MSG_SCHEDULER};
use crate::message_queue::MessageQueue;
use crate::message::Message;
use crate::message_data::MessageData;
use crate::message_error::MessageError;


pub struct MessageBox {
    name: String,
    queue: Arc<Mutex<MessageQueue>>,
    scheduler: Arc<Mutex<MessageScheduler>>,
}

impl MessageBox {
    pub fn new(name: String) -> Result<MessageBox, MessageError> {
        let queue = Arc::new(Mutex::new(MessageQueue::new()));

        match MSG_SCHEDULER.lock() {
            Ok(mut scheduler) => scheduler.add(name.clone(), queue.clone())?,
            _ => return Err(MessageError::MutexError)
        }

        Ok(MessageBox{name, queue,
            scheduler: MSG_SCHEDULER.clone()
        })
    }

    pub fn send(&mut self, receiver: String, data: Vec<MessageData>) {
        match self.scheduler.lock() {
            Ok(mut scheduler) => scheduler.send(Message{sender: self.name.clone(), receiver, data}),
            _ => {}
        }
    }

    pub fn pop(&mut self) -> Option<Message> {
        match self.queue.lock() {
            Ok(mut queue) => queue.pop(),
            _ => None
        }
    }

    pub fn len(&self) -> usize {
        match self.queue.lock() {
            Ok(queue) => queue.len(),
            _ => 0
        }
    }
}
