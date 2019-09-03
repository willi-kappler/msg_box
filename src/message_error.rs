
#[derive(Debug, Clone, PartialEq)]
pub enum MessageError {
    NameAlreadyUsed(String),
    MutexError,
}
