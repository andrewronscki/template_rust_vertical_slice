use dotenv::dotenv;
use lapin::{Connection, ConnectionProperties, Error};
use std::{env, sync::Arc};

use crate::shared::app_state;

pub async fn establish_connection() -> Result<(), Error> {
    dotenv().ok();
    let addr = env::var("AMQP_ADDR").expect("RABBIT URL must be set");
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    println!("RabbitMQ channel created!");

    let shared_channel = Arc::new(channel);
    let _ = app_state::AppState::set_channel(shared_channel).await;

    Ok(())
}
