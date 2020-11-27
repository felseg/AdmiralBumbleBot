#![warn(clippy::all, clippy::needless_pass_by_value)]
#![feature(drain_filter)]
#![feature(str_split_once)]

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
const CACHE_SIZE: usize = 500;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut client = Client::builder(get_env!("ABB_TOKEN"))
        .event_handler(Handler {
            storage: sled::open(STORAGE_PATH).expect("Error opening storage database"),
        })
        .await
        .expect("Error creating client");

    client
        .cache_and_http
        .cache
        .set_max_messages(CACHE_SIZE)
        .await;

    if let Err(e) = client.start().await {
        eprintln!("Error starting client: {}", e);
    }
}
