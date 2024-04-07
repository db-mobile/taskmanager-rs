use lapin::{options::*, Connection, ConnectionProperties, Channel};
use lapin::types::{FieldTable};
use futures_lite::stream::StreamExt;
use crate::models::Message;

pub struct Consumer {
    channel: Channel
}

impl Consumer {
    pub async fn new() -> Consumer {
        let addr = "amqp://spryker:secret@queue.spryker.local/de-docker";
        let conn = Connection::connect(addr, ConnectionProperties::default()).await.unwrap();
        let channel = conn.create_channel().await.unwrap();

        Consumer {
            channel
        }
    }

    pub async fn receive_messages(&self, queue_name: &str, chunk_size: u16) -> Result<Vec<Message>, lapin::Error> {
        let mut consumer = self.channel
            .basic_consume(
                queue_name,
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            ).await?;

        let mut messages = vec![];

        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("error in consumer");

            let message = Message {
                payload: String::from_utf8_lossy(&delivery.data).to_string(),
                delivery_tag: delivery.delivery_tag,
                ack: false,
            };
            messages.push(message);

            if messages.len() >= chunk_size.into() {
                break;
            }
        }

        Ok(messages)
    }

    pub async fn acknowledge(&self, delivery_tag: u64) -> Result<(), lapin::Error> {
        self.channel.basic_ack(delivery_tag, BasicAckOptions::default()).await?;

        Ok(())
    }

    pub async fn reject(&self, delivery_tag: u64) -> Result<(), lapin::Error> {
        self.channel.basic_reject(delivery_tag, BasicRejectOptions::default()).await?;

        Ok(())
    }
}