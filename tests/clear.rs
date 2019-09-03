use msg_box::prelude::*;


#[test]
fn test_clear() {
    let box1 = MessageBox::new("box1_unique").unwrap();

    clear_scheduler();

    let box1 = MessageBox::new("box1_unique");

    match box1 {
        Ok(mut box_unique) => {
            let data1 = vec![MessageData::MsgBool(true)];
            let expected = Message{sender: "box1_unique".to_string(), receiver: "box1_unique".to_string(), data: data1.clone()};
            box_unique.send("box1_unique", data1).unwrap();
            let result = box_unique.pop().unwrap();
            assert_eq!(result, expected);
        }
        Err(_) => unreachable!()
    }
}
