use crate::models::{Message, SyncRequest};
use crate::synchronization_search;

pub fn process_messages(messages: &Vec<Message>) {
    let mut data_write: Vec<(String, String)> = Vec::new();
    let mut data_delete: Vec<String> = Vec::new();

    for message in messages {
        match serde_json::from_str(&message.payload).expect("Error parsing JSON") {
            SyncRequest::Write { write } => {
                let value = write.value.as_object()
                    .expect("Value is not an object")
                    .get("value")
                    .expect("Value does not have a 'value' field")
                    .to_string();

                data_write.push((write.key, value));
            },
            SyncRequest::Delete { delete } => {
                data_delete.push(delete.key);
            },
        }
    }

    if !data_write.is_empty() {
        synchronization_search::write_bulk(&data_write).expect("Error writing to storage");
    }
    if !data_delete.is_empty() {
        synchronization_search::delete_bulk(&data_delete).expect("Error deleting from storage");
    }
}