use lapin::{options::*, types::FieldTable, BasicProperties, Channel};
use std::sync::Arc;

pub async fn init(channel: Arc<Channel>) {
    let queue_options = QueueDeclareOptions {
        durable: true,
        ..Default::default()
    };
    let queue = channel
        .queue_declare("hello", queue_options, FieldTable::default())
        .await
        .unwrap();

    let payload = "Hello, world!";

    channel
        .basic_publish(
            "",
            queue.name().as_str(),
            BasicPublishOptions::default(),
            &payload.as_bytes().to_vec(),
            BasicProperties::default(),
        )
        .await
        .unwrap();

    println!("Sent '{}'", payload);
}
