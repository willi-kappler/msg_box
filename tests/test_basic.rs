use msg_box::{new_msg_box, add_new_receiver, send_message, get_next_message, MsgData};

#[test]
fn create_msg_box() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(16)).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(16))));
}

#[test]
fn test_two_receivers() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();
    add_new_receiver(&mb, "receiver02").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mstring("Hello".to_string())).unwrap();
    send_message(&mb, "sender01", "receiver02", MsgData::Mstring("World".to_string())).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mstring("Hello".to_string()))));

    let result = get_next_message(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mstring("World".to_string()))));
}

#[test]
fn test_two_senders() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();
    add_new_receiver(&mb, "receiver02").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mstring("Hello".to_string())).unwrap();
    send_message(&mb, "sender01", "receiver02", MsgData::Mstring("World".to_string())).unwrap();
    send_message(&mb, "sender02", "receiver01", MsgData::Mu8(9)).unwrap();
    send_message(&mb, "sender02", "receiver02", MsgData::Mu8(12)).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mstring("Hello".to_string()))));

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender02".to_string(), MsgData::Mu8(9))));

    let result = get_next_message(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mstring("World".to_string()))));

    let result = get_next_message(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender02".to_string(), MsgData::Mu8(12))));
}

#[test]
fn test_empty() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();
    add_new_receiver(&mb, "receiver02").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(16)).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(16))));

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, None);

    let result = get_next_message(&mb, "receiver02").unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_types() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();
    add_new_receiver(&mb, "receiver02").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(8)).unwrap();
    send_message(&mb, "sender01", "receiver02", MsgData::Mu16(16)).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mu32(32)).unwrap();
    send_message(&mb, "sender01", "receiver02", MsgData::Mu64(64)).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mbool(true)).unwrap();
    send_message(&mb, "sender01", "receiver02", MsgData::Mchar('X')).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mstring("cool types!".to_string())).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(8))));

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu32(32))));

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mbool(true))));

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mstring("cool types!".to_string()))));

    let result = get_next_message(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu16(16))));

    let result = get_next_message(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu64(64))));

    let result = get_next_message(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mchar('X'))));
}
