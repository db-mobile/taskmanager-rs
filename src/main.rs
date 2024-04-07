use futures::executor;
use crate::consumer::Consumer;

mod synchronization_storage_queue_message_processor;
mod models;
mod consumer;
mod synchronization_storage;
mod synchronization_search_queue_message_processor;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprint!("Missing queue parameter.");
        return;
    }

    let queue = args[1].as_str();

    let consumer = executor::block_on(Consumer::new());

    // TODO: get processor plugin for queue
    let messages = match executor::block_on(consumer.receive_messages(queue, synchronization_storage_queue_message_processor::CHUNK_SIZE)) {
        Ok(messages) => messages,
        Err(e) => {
            eprintln!("Error receiving messages: {:?}", e);
            return;
        }
    };
    synchronization_storage_queue_message_processor::process_messages(&messages);

    for message in messages {
        if message.ack == true {
            executor::block_on(consumer.acknowledge(message.delivery_tag)).expect("Error acknowledging message");
        } else {
            executor::block_on(consumer.reject(message.delivery_tag)).expect("Error rejecting message");
        }
    }
}
