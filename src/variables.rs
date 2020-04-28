use std::env;

pub struct Variables;

impl Variables {
    pub fn log_channel() -> u64 {
        env::var("ABB_LOG_CHANNEL")
            .expect("Log channel not found")
            .parse()
            .expect("Error parsing log channel")
    }

    pub fn token() -> String {
        env::var("ABB_TOKEN").expect("Token not found")
    }
}
