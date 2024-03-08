use lapin::{Connection, ConnectionProperties};
use std::sync::Arc;
use tokio::runtime::Runtime;

use dotenv::dotenv;
use std::env;

mod producer;
mod worker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let rt = Runtime::new()?;
    rt.block_on(async {
        let addr = env::var("AMQP_ADDR").expect("RABBIT URL must be set");
        let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;

        let shared_channel = Arc::new(channel);

        producer::init(shared_channel.clone()).await;
        worker::init(shared_channel.clone()).await;

        Ok(())
    })
}
