use crate::models::Message;
use crate::models::SyncRequest;
use crate::synchronization_storage;

pub const CHUNK_SIZE: u16 = 1000;
pub const KV_PREFIX: &str = "kv:";

pub fn process_messages(messages: &Vec<Message>) {
    let mut data_write: Vec<(String, String)> = Vec::new();
    let mut data_delete: Vec<String> = Vec::new();

    for message in messages {
        match serde_json::from_str(&message.payload).expect("Error parsing JSON") {
            SyncRequest::Write { write } => {
                let key = format!("{}{}", KV_PREFIX, write.key);

                let value = write.value.as_object()
                    .expect("Value is not an object")
                    .get("value")
                    .expect("Value does not have a 'value' field")
                    .to_string();

                data_write.push((key, value));
            },
            SyncRequest::Delete { delete } => {
                let key = format!("{}{}", KV_PREFIX, delete.key);
                data_delete.push(key);
            },
        }
    }

    if !data_write.is_empty() {
        synchronization_storage::write_bulk(&data_write).expect("Error writing to storage");
    }
    if !data_delete.is_empty() {
        synchronization_storage::delete_bulk(&data_delete).expect("Error deleting from storage");
    }
}