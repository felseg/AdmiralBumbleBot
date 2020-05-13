use serde::{Deserialize, Serialize};

const STORAGE_PATH: &str = "storage";

pub fn log_activity(user_id: u64, channel_id: u64, word_count: u16, timestamp: u64) {
    let db = sled::open(STORAGE_PATH).expect("Error opening database");

    db.update_and_fetch(user_id.to_be_bytes(), |value| {
        let mut models: Vec<MessageModel> = match value {
            Some(bytes) => bincode::deserialize(bytes).expect("Error deserializing storage data"),
            None => Vec::new(),
        };

        models.push(MessageModel::new(
            user_id, channel_id, timestamp, word_count,
        ));

        Some(bincode::serialize(&models).expect("Error serializing message data"))
    })
    .expect("Error updating message data");

    db.flush().expect("Error flushing storage tree");
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct MessageModel {
    user_id: u64,
    channel_id: u64,
    timestamp: u64,
    word_count: u16,
}

impl MessageModel {
    pub fn new(user_id: u64, channel_id: u64, timestamp: u64, word_count: u16) -> MessageModel {
        MessageModel {
            user_id,
            channel_id,
            timestamp,
            word_count,
        }
    }
}
