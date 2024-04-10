use std::fs;
use futures::executor;
use serde::Deserialize;
use crate::consumer::Consumer;
use crate::{event_queue_message_processor, synchronization_search_queue_message_processor, synchronization_storage_queue_message_processor};

#[derive(Debug, Deserialize)]
struct QueueConfig {
    name: String,
    processor: String,
    chunk_size: u16,
}

#[derive(Debug, Deserialize)]
struct Config {
    queue: Vec<QueueConfig>,
}

pub fn run(queue_name: &str) {
    let config_path = "Queue.toml";
    let config_content = fs::read_to_string(config_path).expect("Error reading queue config file");
    let config: Config = toml::from_str(&config_content).expect("Error parsing queue config file");
    let queue_config = config
        .queue
        .iter()
        .find(|queue| queue.name == queue_name)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Queue config not found"))
        .expect("Queue config not found");

    let processor = &queue_config.processor;

    let consumer = executor::block_on(Consumer::new());
    let messages = match executor::block_on(consumer.receive_messages(queue_name, queue_config.chunk_size)) {
        Ok(messages) => messages,
        Err(e) => {
            eprintln!("Error receiving messages: {:?}", e);
            return;
        }
    };

    if processor == "synchronization_storage_queue_message_processor" {
        synchronization_storage_queue_message_processor::process_messages(&messages);
    } else if processor == "synchronization_search_queue_message_processor" {
        synchronization_search_queue_message_processor::process_messages(&messages);
    } else if processor == "event_queue_message_processor" {
        event_queue_message_processor::process_messages(&messages);
    } else {
        eprintln!("Unknown processor: {}", processor);
    }

    for message in messages {
        if message.ack == true {
            executor::block_on(consumer.acknowledge(message.delivery_tag)).expect("Error acknowledging message");
        } else {
            executor::block_on(consumer.reject(message.delivery_tag)).expect("Error rejecting message");
        }
    }
}