use msg_box::{MsgData, MsgError, new_msg_box, add_new_receiver, send_message, get_next_message,
    remove_receiver, add_new_group, remove_group, add_receiver_to_group, send_message_to_group};

#[test]
fn receiver_available() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();

    let result = add_new_receiver(&mb, "receiver01");
    assert_eq!(result, Err(MsgError::ReceiverAlreadyAvailable("receiver01".to_string())));
}

#[test]
fn receiver_not_found1() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();

    let result = get_next_message(&mb, "receiver02");
    assert_eq!(result, Err(MsgError::ReceiverNotFound("receiver02".to_string())));
}

#[test]
fn receiver_not_found2() {
    let mb = new_msg_box(64);

    let result = remove_receiver(&mb, "receiver01");
    assert_eq!(result, Err(MsgError::ReceiverNotFound("receiver01".to_string())));
}

#[test]
fn receiver_not_found3() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();

    let result = remove_receiver(&mb, "receiver02");
    assert_eq!(result, Err(MsgError::ReceiverNotFound("receiver02".to_string())));
}

#[test]
fn receiver_not_found4() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, None);

    remove_receiver(&mb, "receiver01").unwrap();

    let result = get_next_message(&mb, "receiver01");
    assert_eq!(result, Err(MsgError::ReceiverNotFound("receiver01".to_string())));
}

#[test]
fn receiver_not_found5() {
    let mb = new_msg_box(64);

    let result = send_message(&mb, "sender01", "receiver01", MsgData::Mu8(16));
    assert_eq!(result, Err(MsgError::ReceiverNotFound("receiver01".to_string())));
}

#[test]
fn receiver_not_found6() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();

    let result = send_message(&mb, "sender01", "receiver02", MsgData::Mu8(16));
    assert_eq!(result, Err(MsgError::ReceiverNotFound("receiver02".to_string())));
}

#[test]
fn receiver_not_found7() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();
    add_new_group(&mb, "group01").unwrap();
    add_receiver_to_group(&mb, "group01", "receiver01").unwrap();
    add_receiver_to_group(&mb, "group01", "receiver02").unwrap();

    let result = send_message_to_group(&mb, "sender01", "group01", MsgData::Mu8(8));
    assert_eq!(result, Err(MsgError::ReceiverNotFound("receiver02".to_string())));

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(8))));
}

#[test]
fn group_available() {
    let mb = new_msg_box(64);

    add_new_group(&mb, "group01").unwrap();
    let result = add_new_group(&mb, "group01");
    assert_eq!(result, Err(MsgError::GroupAlreadyAvailable("group01".to_string())));
}

#[test]
fn group_not_found1() {
    let mb = new_msg_box(64);

    let result = remove_group(&mb, "group01");
    assert_eq!(result, Err(MsgError::GroupNotFound("group01".to_string())));
}

#[test]
fn group_not_found2() {
    let mb = new_msg_box(64);

    add_new_group(&mb, "group01").unwrap();
    let result = remove_group(&mb, "group02");

    assert_eq!(result, Err(MsgError::GroupNotFound("group02".to_string())));
}

#[test]
fn group_not_found3() {
    let mb = new_msg_box(64);

    add_new_group(&mb, "group01").unwrap();
    remove_group(&mb, "group01").unwrap();
    let result = remove_group(&mb, "group01");

    assert_eq!(result, Err(MsgError::GroupNotFound("group01".to_string())));
}

#[test]
fn group_not_found4() {
    let mb = new_msg_box(64);

    let result = send_message_to_group(&mb, "sender01", "group01", MsgData::Mu8(8));

    assert_eq!(result, Err(MsgError::GroupNotFound("group01".to_string())));
}

#[test]
fn group_not_found5() {
    let mb = new_msg_box(64);

    let result = add_receiver_to_group(&mb, "group01", "receiver01");

    assert_eq!(result, Err(MsgError::GroupNotFound("group01".to_string())));
}

#[test]
fn group_not_found6() {
    let mb = new_msg_box(64);

    add_new_group(&mb, "group01").unwrap();
    let result = add_receiver_to_group(&mb, "group02", "receiver01");

    assert_eq!(result, Err(MsgError::GroupNotFound("group02".to_string())));
}
