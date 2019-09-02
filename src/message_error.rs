
#[derive(Debug)]
pub enum MessageError {
    NameAlreadyUsed(String),
    MutexError,
}
