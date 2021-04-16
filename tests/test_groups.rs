use msg_box::{new_msg_box, add_new_receiver, add_new_group, add_receiver_to_group, send_message, send_message_to_group, get_next_message};

#[test]
fn test_group1() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();
    add_new_receiver(&mb, "receiver02").unwrap();
    add_new_receiver(&mb, "receiver03").unwrap();

    add_new_group(&mb, "group1").unwrap();
    add_receiver_to_group(&mb, "group1", "receiver01").unwrap();
    add_receiver_to_group(&mb, "group1", "receiver02").unwrap();
    add_receiver_to_group(&mb, "group1", "receiver03").unwrap();

    send_message(&mb, "sender01", "receiver01", 16_u8).unwrap();
    send_message(&mb, "sender01", "receiver02", 16_u8).unwrap();
    send_message(&mb, "sender01", "receiver03", 16_u8).unwrap();

    send_message_to_group(&mb, "sender01", "group1", 17_u8).unwrap();

    let result = get_next_message::<u8>(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), 16_u8)));

    let result = get_next_message::<u8>(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), 16_u8)));

    let result = get_next_message::<u8>(&mb, "receiver03").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), 16_u8)));

    let result = get_next_message::<u8>(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), 17_u8)));

    let result = get_next_message::<u8>(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), 17_u8)));

    let result = get_next_message::<u8>(&mb, "receiver03").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), 17_u8)));
}

#[test]
fn test_max_size() {
    let mb = new_msg_box(3);

    add_new_receiver(&mb, "receiver01").unwrap();
    add_new_receiver(&mb, "receiver02").unwrap();
    add_new_receiver(&mb, "receiver03").unwrap();

    add_new_group(&mb, "group1").unwrap();
    add_receiver_to_group(&mb, "group1", "receiver01").unwrap();
    add_receiver_to_group(&mb, "group1", "receiver02").unwrap();
    add_receiver_to_group(&mb, "group1", "receiver03").unwrap();

    send_message_to_group(&mb, "sender01", "group1", 20_u8).unwrap();
    send_message_to_group(&mb, "sender01", "group1", 21_u8).unwrap();
    send_message_to_group(&mb, "sender01", "group1", 22_u8).unwrap();
    send_message_to_group(&mb, "sender01", "group1", 23_u8).unwrap();

    let result = get_next_message::<u8>(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), 20_u8)));

    let result = get_next_message::<u8>(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), 20_u8)));

    let result = get_next_message::<u8>(&mb, "receiver03").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), 20_u8)));
}
