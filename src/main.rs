use {handler::Handler, variables::Variables};

use serenity::Client;

mod commands;
mod handler;
mod variables;

const CACHE_SIZE: usize = 100;

fn main() {
    let mut client = Client::new(Variables::token(), Handler).expect("Error creating client");

    {
        let mut cache = client.cache_and_http.cache.write();
        cache.settings_mut().max_messages(CACHE_SIZE);
    }

    if let Err(e) = client.start() {
        eprintln!("Error starting client: {}", e);
    }
}
