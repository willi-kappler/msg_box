mod msg_box;

pub use msg_box::{MsgData, MsgBox, new_msg_box, add_new_receiver, remove_receiver, send_message, get_next_message,
    add_new_group, remove_group, send_message_to_group
};
