

mod message_scheduler;
mod message_queue;
mod message_box;
mod message;
mod message_data;
mod message_error;

pub mod prelude {
    pub use crate::message_box::MessageBox;
    pub use crate::message::Message;
    pub use crate::message_data::MessageData;
    pub use crate::message_scheduler::clear_scheduler;
    pub use crate::message_error::MessageError;
}
