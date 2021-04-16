use msg_box::{new_msg_box, add_new_receiver, send_message, get_next_message};

#[test]
fn create_msg_box() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();

    send_message(&mb, "sender01", "receiver01", 16_u8).unwrap();

    let result = get_next_message::<u8>(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new(16))));
}

#[test]
fn test_two_receivers() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();
    add_new_receiver(&mb, "receiver02").unwrap();

    send_message(&mb, "sender01", "receiver01", "Hello".to_string()).unwrap();
    send_message(&mb, "sender01", "receiver02", "World".to_string()).unwrap();

    let result = get_next_message::<String>(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new("Hello".to_string()))));

    let result = get_next_message::<String>(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new("World".to_string()))));
}

#[test]
fn test_two_senders() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();
    add_new_receiver(&mb, "receiver02").unwrap();

    send_message(&mb, "sender01", "receiver01", "Hello".to_string()).unwrap();
    send_message(&mb, "sender01", "receiver02", "World".to_string()).unwrap();
    send_message(&mb, "sender02", "receiver01", 9_u16).unwrap();
    send_message(&mb, "sender02", "receiver02", 12_u16).unwrap();

    let result = get_next_message::<String>(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new("Hello".to_string()))));

    let result = get_next_message::<u16>(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender02".to_string(), Box::new(9))));

    let result = get_next_message::<String>(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new("World".to_string()))));

    let result = get_next_message::<u16>(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender02".to_string(), Box::new(12))));
}

#[test]
fn test_empty() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();
    add_new_receiver(&mb, "receiver02").unwrap();

    send_message(&mb, "sender01", "receiver01", 16_u32).unwrap();

    let result = get_next_message::<u32>(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new(16))));

    let result = get_next_message::<u8>(&mb, "receiver01").unwrap();
    assert_eq!(result, None);

    let result = get_next_message::<u64>(&mb, "receiver02").unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_types() {
    let mb = new_msg_box(64);

    add_new_receiver(&mb, "receiver01").unwrap();
    add_new_receiver(&mb, "receiver02").unwrap();

    send_message(&mb, "sender01", "receiver01", true).unwrap();
    send_message(&mb, "sender01", "receiver02", 8_u8).unwrap();
    send_message(&mb, "sender01", "receiver01", 16_u16).unwrap();
    send_message(&mb, "sender01", "receiver02", 32_u32).unwrap();
    send_message(&mb, "sender01", "receiver01", 64_u64).unwrap();
    send_message(&mb, "sender01", "receiver02", 32.1_f32).unwrap();
    send_message(&mb, "sender01", "receiver01", 64.2_f64).unwrap();
    send_message(&mb, "sender01", "receiver02", 'X').unwrap();
    send_message(&mb, "sender01", "receiver01", "cool types!".to_string()).unwrap();
    send_message(&mb, "sender01", "receiver02", vec![(50_u8, false, 'R'), (12_u8, true, 'A')]).unwrap();

    let result = get_next_message::<bool>(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new(true))));

    let result = get_next_message::<u8>(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new(8))));

    let result = get_next_message::<u16>(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new(16))));

    let result = get_next_message::<u32>(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new(32))));

    let result = get_next_message::<u64>(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new(64))));

    let result = get_next_message::<f32>(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new(32.1))));

    let result = get_next_message::<f64>(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new(64.2))));

    let result = get_next_message::<char>(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new('X'))));

    let result = get_next_message::<String>(&mb, "receiver01").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new("cool types!".to_string()))));

    let result = get_next_message::<Vec<(u8, bool, char)>>(&mb, "receiver02").unwrap();
    assert_eq!(result, Some(("sender01".to_string(), Box::new(vec![(50_u8, false, 'R'), (12_u8, true, 'A')]))));
}
