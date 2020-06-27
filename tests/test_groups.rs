use msg_box::{new_msg_box, add_new_receiver, add_new_group, add_receiver_to_group, send_message, send_message_to_group, get_next_message, MsgData};

#[test]
fn test_group1() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();
    add_new_receiver(&mb, "receiver02").unwrap();
    add_new_receiver(&mb, "receiver03").unwrap();
    add_new_receiver(&mb, "sender01").unwrap();

    add_new_group(&mb, "group1").unwrap();
    add_receiver_to_group(&mb, "group1", "receiver01").unwrap();
    add_receiver_to_group(&mb, "group1", "receiver02").unwrap();
    add_receiver_to_group(&mb, "group1", "receiver03").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(16)).unwrap();
    send_message(&mb, "sender01", "receiver02", MsgData::Mu8(16)).unwrap();
    send_message(&mb, "sender01", "receiver03", MsgData::Mu8(16)).unwrap();

    send_message_to_group(&mb, "sender01", "group1", MsgData::Mu8(17)).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(16))));

    let result = get_next_message(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(16))));

    let result = get_next_message(&mb, "receiver03").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(16))));

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(17))));

    let result = get_next_message(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(17))));

    let result = get_next_message(&mb, "receiver03").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(17))));
}
