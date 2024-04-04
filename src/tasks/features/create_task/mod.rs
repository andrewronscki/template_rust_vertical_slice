pub mod command;
pub mod model;
pub mod repository;
pub mod route;

use futures_util::StreamExt;
use lapin::{options::*, types::FieldTable};

use crate::{
    shared::app_state::AppState,
    tasks::features::create_task::command::{CommandHandler, CreateTaskCommand},
};

pub async fn init() {
    let channel_arc = {
        let locked_channel_option = AppState::get_instance()
            .channel
            .lock()
            .expect("Failed to lock the mutex");
        locked_channel_option.clone() // Clona o Arc, não o Channel em si.
    };

    if let Some(channel) = channel_arc {
        let queue_options = QueueDeclareOptions {
            durable: true,
            ..Default::default()
        };
        let queue = channel
            .queue_declare("tasks.create", queue_options, FieldTable::default())
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
        log::info!("Waiting for messages on queue: {}", queue.name().as_str());

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                match serde_json::from_slice::<CreateTaskCommand>(&delivery.data) {
                    Ok(command) => {
                        let handler = CommandHandler::new();
                        match handler.command(command) {
                            Ok(account) => {
                                log::info!("Task created: {:?}", account);
                            }
                            Err(e) => {
                                log::error!("Error on create task: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Error deserializing message: {:?}", e);
                    }
                }

                let ack_result = delivery.ack(BasicAckOptions::default()).await;
                if let Err(e) = ack_result {
                    log::error!("Error acknowledging message: {:?}", e);
                }
            }
        }
    }
}
