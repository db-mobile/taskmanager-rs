mod synchronization_storage_queue_message_processor;
mod synchronization_search_queue_message_processor;
mod event_queue_message_processor;
mod models;
mod consumer;
mod synchronization_storage;

mod task_manager;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprint!("Missing queue parameter.");
        return;
    }

    let queue_name = args[1].as_str();

    task_manager::run(queue_name);
}
