use serde::{Deserialize, Serialize};

pub fn log_activity(user_id: u64, channel_id: u64, word_count: u16, timestamp: u64, db: &sled::Db) {
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

pub fn get_user_message_data(user_id: u64, db: &sled::Db) -> Vec<MessageModel> {
    let data: Vec<MessageModel> = match db
        .get(user_id.to_be_bytes())
        .expect("Error retrieving message data from storage")
    {
        Some(bytes) => bincode::deserialize(&bytes).unwrap(),
        None => Vec::new(),
    };
    data
}

pub fn pass_jenkem(recipient: u64, db: &sled::Db) -> i32 {
    let model = db
        .update_and_fetch("jenkem", |value| {
            let mut model = match value {
                Some(bytes) => bincode::deserialize(&bytes).unwrap(),
                None => JenkemModel::new(0),
            };

            model.pass(recipient);

            Some(bincode::serialize(&model).expect("Error passing jenkem"))
        })
        .expect("Error passing jenkem");

    db.flush().expect("Error flushing storage tree");

    let model: JenkemModel = match model {
        Some(bytes) => bincode::deserialize(&bytes).unwrap(),
        None => JenkemModel::new(0),
    };

    model.huff_count
}

pub fn init_jenkem(brewer: u64, db: &sled::Db) {
    db.update_and_fetch("jenkem", |value| {
        let mut model = match value {
            Some(bytes) => bincode::deserialize(&bytes).unwrap(),
            None => JenkemModel::new(0),
        };

        model.current_holder = brewer;
        model.huff_count = 0;

        Some(bincode::serialize(&model).expect("Error brewing jenkem"))
    })
    .expect("Error brewing jenkem");

    db.flush().expect("Error flushing storage tree");
}

pub fn get_jenkem_holder(db: &sled::Db) -> u64 {
    let jenkem = match db.get("jenkem").expect("Error retrieving jenkem") {
        Some(bytes) => bincode::deserialize(&bytes).unwrap(),
        None => JenkemModel::new(0),
    };

    jenkem.current_holder
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct MessageModel {
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

    pub fn _timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn word_count(&self) -> u16 {
        self.word_count
    }

    pub fn channel_id(&self) -> u64 {
        self.channel_id
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct JenkemModel {
    current_holder: u64,
    huff_count: i32,
}

impl JenkemModel {
    pub fn new(current_holder: u64) -> JenkemModel {
        JenkemModel {
            current_holder: current_holder,
            huff_count: 0,
        }
    }

    pub fn pass(&mut self, recipient: u64) {
        self.huff_count += 1;
        self.current_holder = recipient;
    }
}
