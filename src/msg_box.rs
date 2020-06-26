use std::sync::{Arc, Mutex, MutexGuard};

// use log::{error, debug};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MsgData {
    M_u8(u8),
    M_u16(u16),
    M_u32(u32),
    M_u64(u64),
    M_bool(bool),
    M_char(char),
    M_string(String),
}
#[derive(Debug, Clone)]
pub struct MsgBoxIntern {
    max_size: usize,
    queue: Vec<(String, Vec<(String, MsgData)>)>,
    groups: Vec<(String, Vec<String>)>,
}

pub type MsgBox = Arc<Mutex<MsgBoxIntern>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MsgError {
    ReceiverNotFound(String),
    SenderNotFound(String),
    ReceiverAlreadyAvailable(String),
    GroupNotFound(String),
    GroupAlreadyAvailable(String),
    CouldNotLockMutex,
}

fn get_receiver_index(msg_box: &MutexGuard<MsgBoxIntern>, receiver: &str) -> Option<usize> {
    for (i, item) in msg_box.queue.iter().enumerate() {
        if item.0 == receiver {
            return Some(i)
        }
    }

    None
}

fn has_receiver(msg_box: &MutexGuard<MsgBoxIntern>, receiver: &str) -> bool {
    get_receiver_index(msg_box, receiver).is_some()
}

fn get_group_index(msg_box: &MutexGuard<MsgBoxIntern>, group: &str) -> Option<usize> {
    // TODO
    None
}

fn has_group(msg_box: &MutexGuard<MsgBoxIntern>, group: &str) -> bool {
    // TODO
    get_group_index(msg_box, group).is_some()
}

pub fn new_msg_box(max_size: usize) -> MsgBox {
    let msg_box = MsgBoxIntern {
        max_size,
        queue: Vec::new(),
        groups: Vec::new(),
    };

    Arc::new(Mutex::new(msg_box))
}

pub fn add_new_receiver(msg_box: &MsgBox, receiver: &str) -> Result<(), MsgError> {
    match msg_box.lock() {
        Ok(mut msg_box) => {
            if has_receiver(&msg_box, receiver) {
                Err(MsgError::ReceiverAlreadyAvailable(receiver.to_string()))
            } else {
                msg_box.queue.push((receiver.to_string(), Vec::new()));
                Ok(())
            }
        }
        Err(e) => {
            Err(MsgError::CouldNotLockMutex)
        }
    }
}

pub fn remove_receiver(msg_box: &MsgBox, receiver: &str) -> Result<(), MsgError> {
    match msg_box.lock() {
        Ok(mut msg_box) => {
            match get_receiver_index(&msg_box, receiver) {
                Some(i) => {
                    msg_box.queue.remove(i);
                    Ok(())
                }
                None => {
                    Err(MsgError::ReceiverNotFound(receiver.to_string()))
                }
            }
        }
        Err(e) => {
            Err(MsgError::CouldNotLockMutex)
        }
    }
}

pub fn add_new_group(msg_box: &MsgBox, group: &str) -> Result<(), MsgError> {
    match msg_box.lock() {
        Ok(mut msg_box) => {
            // TODO
            Ok(())
        }
        Err(e) => {
            Err(MsgError::CouldNotLockMutex)
        }
    }
}

pub fn remove_group(msg_box: &MsgBox, group: &str) -> Result<(), MsgError> {
    match msg_box.lock() {
        Ok(mut msg_box) => {
            // TODO
            Ok(())
        }
        Err(e) => {
            Err(MsgError::CouldNotLockMutex)
        }
    }
}

pub fn add_receiver_to_group(msg_box: &MsgBox, group: &str, receiver: &str) -> Result<(), MsgError> {
    match msg_box.lock() {
        Ok(mut msg_box) => {
            // TODO
            Ok(())
        }
        Err(e) => {
            Err(MsgError::CouldNotLockMutex)
        }
    }
}

pub fn send_message(msg_box: &MsgBox, sender: &str, receiver: &str, message: MsgData) -> Result<(), MsgError> {
    match msg_box.lock() {
        Ok(mut msg_box) => {
            if has_receiver(&msg_box, sender) {
                match get_receiver_index(&msg_box, receiver) {
                    Some(i) => {
                        msg_box.queue[i].1.insert(0, (sender.to_string(), message));
                        return Ok(())
                    }
                    None => {
                        Err(MsgError::ReceiverNotFound(receiver.to_string()))
                    }
                }
            } else {
                Err(MsgError::SenderNotFound(sender.to_string()))
            }
        }
        Err(e) => {
            Err(MsgError::CouldNotLockMutex)
        }
    }
}

pub fn send_message_to_group(msg_box: &MsgBox, sender: &str, group: &str, message: MsgData) -> Result<(), MsgError> {
    match msg_box.lock() {
        Ok(mut msg_box) => {
            if has_receiver(&msg_box, sender) {
                // TODO
                Ok(())
            } else {
                Err(MsgError::GroupNotFound(sender.to_string()))
            }
        }
        Err(e) => {
            Err(MsgError::CouldNotLockMutex)
        }
    }
}

pub fn get_next_message(msg_box: &MsgBox, receiver: &str) -> Result<Option<(String, MsgData)>, MsgError> {
    match msg_box.lock() {
        Ok(mut msg_box) => {
            match get_receiver_index(&msg_box, receiver) {
                Some(i) => {
                    Ok(msg_box.queue[i].1.pop())
                }
                None => {
                    Err(MsgError::ReceiverNotFound(receiver.to_string()))
                }
            }
        }
        Err(e) => {
            Err(MsgError::CouldNotLockMutex)
        }
    }
}

