#![warn(clippy::all, clippy::needless_pass_by_value)]
#![feature(drain_filter)]

use handler::Handler;

use dotenv::dotenv;
use serenity::Client;

#[macro_use]
mod macros;
mod commands;
mod handler;
mod logging;
mod pastas;
mod storage;

const STORAGE_PATH: &str = "storage";

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut client = Client::builder(get_env!("ABB_TOKEN"))
        .event_handler(Handler {
            storage: sled::open(STORAGE_PATH).expect("Error opening storage database"),
        })
        .await
        .expect("Error creating client");

    client.cache_and_http.cache.set_max_messages(100).await;

    if let Err(e) = client.start().await {
        eprintln!("Error starting client: {}", e);
    }
}
