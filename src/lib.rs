mod msg_box;

pub use crate::msg_box::{MsgBox, MsgError, new_msg_box, add_new_receiver, remove_receiver, send_message, get_next_message,
    add_new_group, remove_group, add_receiver_to_group, send_message_to_group
};
