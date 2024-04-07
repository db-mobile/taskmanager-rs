use crate::models::Message;

pub fn process_messages(messages: &Vec<Message>) {
    for message in messages {
        println!("Message: {:?}", message.payload);
    }
}