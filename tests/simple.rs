
use msg_box::prelude::*;


#[test]
fn empty() {
    let mut box3 = MessageBox::new("box3").unwrap();

    let result = box3.pop();

    assert_eq!(result, None);
}

#[test]
fn name_in_use() {
    let box_name = "box4".to_string();
    let _box1 = MessageBox::new(&box_name).unwrap();

    match MessageBox::new(&box_name) {
        Err(e) => assert_eq!(e, MessageError::NameAlreadyUsed(box_name)),
        Ok(_) => unreachable!()
    }
}

#[test]
fn simple1() {
    let mut box1 = MessageBox::new("box1").unwrap();
    let mut box2 = MessageBox::new("box2").unwrap();

    let data1 = vec![MessageData::MsgString("Hello World!".to_string())];
    let data2 = data1.clone();

    box1.send("box2", data1);
    let result = box2.pop().unwrap();
    let expected = Message{sender: "box1".to_string(), receiver: "box2".to_string(), data: data2};

    assert_eq!(result, expected);
}

#[test]
fn simple2() {
    let mut box5 = MessageBox::new("box5").unwrap();
    let mut box6 = MessageBox::new("box6").unwrap();

    let data1 = vec![MessageData::MsgU8(44), MessageData::MsgU8(55)];
    let data2 = vec![MessageData::MsgU8(90), MessageData::MsgU8(20)];
    let data3 = data1.clone();
    let data4 = data2.clone();

    box5.send("box6", data1);
    let result = box6.pop().unwrap();
    let expected = Message{sender: "box5".to_string(), receiver: "box6".to_string(), data: data3};

    assert_eq!(result, expected);


    box5.send("box6", data2);
    let result = box6.pop().unwrap();
    let expected = Message{sender: "box5".to_string(), receiver: "box6".to_string(), data: data4};

    assert_eq!(result, expected);
}

#[test]
fn simple3() {
    fn inc_data(message: Message) -> Vec<MessageData> {
        let mut result = Vec::new();

        for item in message.data.iter() {
            match item {
                MessageData::MsgU8(value) => result.push(MessageData::MsgU8(value + 1)),
                data => result.push(data.clone())
            }
        }

        result
    }

    let mut box7 = MessageBox::new("box7").unwrap();
    let mut box8 = MessageBox::new("box8").unwrap();
    let mut box9 = MessageBox::new("box9").unwrap();

    let data1 = vec![MessageData::MsgU8(1), MessageData::MsgBool(true), MessageData::MsgU8(2)];
    let data2 = vec![MessageData::MsgU8(4), MessageData::MsgBool(true), MessageData::MsgU8(5)];

    box7.send("box8", data1);
    let result = box8.pop().unwrap();
    box8.send("box9", inc_data(result));
    let result = box9.pop().unwrap();
    box9.send("box7", inc_data(result));
    let result = box7.pop().unwrap();
    box7.send("box7", inc_data(result));
    let result = box7.pop().unwrap();

    let expected = Message{sender: "box7".to_string(), receiver: "box7".to_string(), data: data2};

    assert_eq!(result, expected);
}

#[test]
fn test_len() {
    let mut box10 = MessageBox::new("box10").unwrap();
    let data1 = vec![MessageData::MsgBool(true)];

    assert_eq!(box10.len(), 0);

    box10.send("box10", data1.clone());

    assert_eq!(box10.len(), 1);

    box10.send("box10", data1.clone());

    assert_eq!(box10.len(), 2);

    box10.send("box10", data1.clone());

    assert_eq!(box10.len(), 3);

    box10.pop();

    assert_eq!(box10.len(), 2);

    box10.pop();

    assert_eq!(box10.len(), 1);

    box10.pop();

    assert_eq!(box10.len(), 0);
}
