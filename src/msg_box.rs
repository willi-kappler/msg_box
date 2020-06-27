use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

// use log::{error, debug};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MsgData {
    Mu8(u8),
    Mu16(u16),
    Mu32(u32),
    Mu64(u64),
    Mbool(bool),
    Mchar(char),
    Mstring(String),
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
    ReceiverAlreadyAvailable(String),
    GroupNotFound(String),
    GroupAlreadyAvailable(String),
    CouldNotLockMutex,
}

impl From<PoisonError<MutexGuard<'_, MsgBoxIntern>>> for MsgError {
    fn from(_err: PoisonError<MutexGuard<MsgBoxIntern>>) -> MsgError {
        MsgError::CouldNotLockMutex
    }
}

fn get_receiver_index(msg_box: &MutexGuard<MsgBoxIntern>, receiver: &str) -> Result<usize, MsgError> {
    for (i, item) in msg_box.queue.iter().enumerate() {
        if item.0 == receiver {
            return Ok(i)
        }
    }

    Err(MsgError::ReceiverNotFound(receiver.to_string()))
}

fn has_receiver(msg_box: &MutexGuard<MsgBoxIntern>, receiver: &str) -> bool {
    get_receiver_index(msg_box, receiver).is_ok()
}

fn get_group_index(msg_box: &MutexGuard<MsgBoxIntern>, group: &str) -> Result<usize, MsgError> {
    for (i, item) in msg_box.groups.iter().enumerate() {
        if item.0 == group {
            return Ok(i)
        }
    }

    Err(MsgError::GroupNotFound(group.to_string()))
}

fn has_group(msg_box: &MutexGuard<MsgBoxIntern>, group: &str) -> bool {
    get_group_index(msg_box, group).is_ok()
}

pub fn new_msg_box(max_size: usize) -> MsgBox {
    let msg_box = MsgBoxIntern {
        max_size,
        queue: Vec::new(), // TODO: with capacity
        groups: Vec::new(),
    };

    Arc::new(Mutex::new(msg_box))
}

pub fn add_new_receiver(msg_box: &MsgBox, receiver: &str) -> Result<(), MsgError> {
    let mut msg_box = msg_box.lock()?;

    if has_receiver(&msg_box, receiver) {
        Err(MsgError::ReceiverAlreadyAvailable(receiver.to_string()))
    } else {
        msg_box.queue.push((receiver.to_string(), Vec::new()));
        Ok(())
    }
}

pub fn remove_receiver(msg_box: &MsgBox, receiver: &str) -> Result<(), MsgError> {
    let mut msg_box = msg_box.lock()?;
    let i = get_receiver_index(&msg_box, receiver)?;

    msg_box.queue.remove(i);

    Ok(())
}

pub fn add_new_group(msg_box: &MsgBox, group: &str) -> Result<(), MsgError> {
    let mut msg_box = msg_box.lock()?;

    if has_group(&msg_box, group) {
        Err(MsgError::GroupAlreadyAvailable(group.to_string()))
    } else {
        msg_box.groups.push((group.to_string(), Vec::new()));
        Ok(())
    }
}

pub fn remove_group(msg_box: &MsgBox, group: &str) -> Result<(), MsgError> {
    let mut msg_box = msg_box.lock()?;
    let i = get_group_index(&msg_box, group)?;

    msg_box.groups.remove(i);

    Ok(())
}

pub fn add_receiver_to_group(msg_box: &MsgBox, group: &str, receiver: &str) -> Result<(), MsgError> {
    let mut msg_box = msg_box.lock()?;
    let i = get_group_index(&msg_box, group)?;

    msg_box.groups[i].1.push(receiver.to_string());

    Ok(())
}

fn send_message_intern(msg_box: &mut MutexGuard<MsgBoxIntern>, sender: &str, receiver: &str, message: MsgData) -> Result<(), MsgError> {
    let i = get_receiver_index(&msg_box, receiver)?;

    msg_box.queue[i].1.insert(0, (sender.to_string(), message));

    let max_size = msg_box.max_size;

    msg_box.queue[i].1.truncate(max_size);

    Ok(())
}

pub fn send_message(msg_box: &MsgBox, sender: &str, receiver: &str, message: MsgData) -> Result<(), MsgError> {
    let mut msg_box = msg_box.lock()?;

    send_message_intern(&mut msg_box, sender, receiver, message)
}

pub fn send_message_to_group(msg_box: &MsgBox, sender: &str, group: &str, message: MsgData) -> Result<(), MsgError> {
    let mut msg_box = msg_box.lock()?;
    let i = get_group_index(&msg_box, group)?;

    // TODO: Remove clone(), use Rc, RefCell, or s.th. similar
    let groups = msg_box.groups[i].clone();

    for receiver in groups.1.iter() {
        send_message_intern(&mut msg_box, sender, receiver, message.clone())?
    }

    Ok(())
}

pub fn get_next_message(msg_box: &MsgBox, receiver: &str) -> Result<Option<(String, MsgData)>, MsgError> {
    let mut msg_box = msg_box.lock()?;
    let i = get_receiver_index(&msg_box, receiver)?;

    Ok(msg_box.queue[i].1.pop())
}
