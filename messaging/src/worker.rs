use std::sync::Arc;

use futures_util::StreamExt;
use lapin::{options::*, types::FieldTable, Channel};

pub async fn init(channel: Arc<Channel>) {
    let queue_options = QueueDeclareOptions {
        durable: true,
        ..Default::default()
    };
    let queue = channel
        .queue_declare("hello", queue_options, FieldTable::default())
        .await
        .unwrap();
    let mut consumer = channel
        .basic_consume(
            queue.name().as_str(),
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();
    println!("Waiting for messages. To exit press CTRL+C");

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            println!("Received: {:?}", String::from_utf8_lossy(&delivery.data));
            let ack_result = delivery.ack(BasicAckOptions::default()).await;
            if let Err(e) = ack_result {
                eprintln!("Error acknowledging message: {:?}", e);
            }
        }
    }
}
