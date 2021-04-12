use msg_box::{new_msg_box, add_new_receiver, send_message, get_next_message};

#[test]
fn test_max_size1_1() {
    let mb = new_msg_box(1);

    add_new_receiver(&mb, "receiver01").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(16)).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(16))));
}

#[test]
fn test_max_size1_2() {
    let mb = new_msg_box(1);

    add_new_receiver(&mb, "receiver01").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(16)).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(17)).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(17))));
}

#[test]
fn test_max_size1_3() {
    let mb = new_msg_box(1);

    add_new_receiver(&mb, "receiver01").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(16)).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(17)).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(18)).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(18))));
}

#[test]
fn test_max_size2_1() {
    let mb = new_msg_box(2);

    add_new_receiver(&mb, "receiver01").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(16)).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(16))));
}

#[test]
fn test_max_size2_2() {
    let mb = new_msg_box(2);

    add_new_receiver(&mb, "receiver01").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(16)).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(17)).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(16))));
}

#[test]
fn test_max_size2_3() {
    let mb = new_msg_box(2);

    add_new_receiver(&mb, "receiver01").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(16)).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(17)).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(18)).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(17))));
}

#[test]
fn test_max_size3_1() {
    let mb = new_msg_box(3);

    add_new_receiver(&mb, "receiver01").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(16)).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(16))));
}

#[test]
fn test_max_size3_2() {
    let mb = new_msg_box(3);

    add_new_receiver(&mb, "receiver01").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(16)).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(17)).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(16))));
}

#[test]
fn test_max_size3_3() {
    let mb = new_msg_box(3);

    add_new_receiver(&mb, "receiver01").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(16)).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(17)).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(18)).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(16))));
}

#[test]
fn test_max_size3_4() {
    let mb = new_msg_box(3);

    add_new_receiver(&mb, "receiver01").unwrap();

    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(16)).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(17)).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(18)).unwrap();
    send_message(&mb, "sender01", "receiver01", MsgData::Mu8(19)).unwrap();

    let result = get_next_message(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), MsgData::Mu8(17))));
}
