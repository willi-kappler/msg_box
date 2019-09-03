// Rust modules
use std::sync::{Arc, Mutex};

// Internal modules
use crate::message_queue::MessageQueue;
use crate::message_error::MessageError;
use crate::message::Message;

// External modules
use lazy_static::lazy_static;


lazy_static! {
    pub(crate) static ref MSG_SCHEDULER: Arc<Mutex<MessageScheduler>> = {
        Arc::new(Mutex::new(MessageScheduler::new()))
    };
}


pub(crate) struct MessageScheduler {
    queues: Vec<(String, Arc<Mutex<MessageQueue>>)>
}

impl MessageScheduler {
    fn new() -> MessageScheduler {
        MessageScheduler{queues: Vec::new()}
    }

    pub(crate) fn add(&mut self, new_name: String, queue: Arc<Mutex<MessageQueue>>) -> Result<(), MessageError> {
        for (name, _) in self.queues.iter() {
            if *name == new_name {
                return Err(MessageError::NameAlreadyUsed(new_name))
            }
        }

        self.queues.push((new_name, queue));

        Ok(())
    }

    pub(crate) fn send(&mut self, message: Message) -> Result<(), MessageError> {
        let receiver = message.receiver.clone();

        for (name, queue) in self.queues.iter_mut() {
            if *name == receiver {
                match queue.lock() {
                    Ok(mut queue) => {
                        queue.add(message);
                        return Ok(())
                    }
                    _ => return Err(MessageError::MutexError)
                }
            }
        }

        Err(MessageError::ReceiverNotFound(receiver))
    }

    pub(crate) fn clear(&mut self) {
        self.queues.clear();
    }
}

pub fn clear_scheduler() -> Result<(), MessageError> {
    match MSG_SCHEDULER.lock() {
        Ok(mut scheduler) => Ok(scheduler.clear()),
        _ => Err(MessageError::MutexError)
    }
}
