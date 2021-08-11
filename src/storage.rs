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
                Some(bytes) => bincode::deserialize(bytes).unwrap(),
                None => JenkemModel::new(0),
            };

            model.pass(recipient);

            Some(bincode::serialize(&model).expect("Error passing jenkem"))
        })
        .expect("Error passing jenkem");

    let model: JenkemModel = match model {
        Some(bytes) => bincode::deserialize(&bytes).unwrap(),
        None => JenkemModel::new(0),
    };

    model.huff_count()
}

pub fn reject_jenkem(db: &sled::Db) -> Result<(), ()> {
    let model = db
        .fetch_and_update("jenkem", |value| {
            let mut model = match value {
                Some(bytes) => bincode::deserialize(bytes).unwrap(),
                None => JenkemModel::new(0),
            };

            if model.current_holder() != 0 && model.previous_holder() != 0 {
                model.reject();
            };

            Some(bincode::serialize(&model).expect("Error rejecting jenkem"))
        })
        .expect("Error rejecting jenkem");

    let model = match model {
        Some(bytes) => bincode::deserialize(&bytes).unwrap(),
        None => JenkemModel::new(0),
    };

    if model.previous_holder() == 0 {
        Err(())
    } else {
        Ok(())
    }
}

pub fn locate_jenkem(db: &sled::Db) -> u64 {
    let model = match db.get("jenkem").expect("Error locating jenkem") {
        Some(bytes) => bincode::deserialize(&bytes).unwrap(),
        None => JenkemModel::new(0),
    };

    model.current_holder()
}

pub fn init_jenkem(brewer: u64, db: &sled::Db) {
    db.remove("jenkem").expect("Error deleting jenkem");
    db.insert(
        "jenkem",
        bincode::serialize(&JenkemModel::new(brewer)).expect("Error serializing jenkem"),
    )
    .expect("Error inserting jenkem");
    db.flush().expect("Error flushing storage tree");
}

pub fn update_jenkem_streak(streak: i32, db: &sled::Db) {
    db.update_and_fetch("jenkem_streak", |value| {
        let current_streak = match value {
            Some(bytes) => bincode::deserialize(bytes).expect("Error deserializing jenkem streak"),
            None => 0,
        };

        if streak > current_streak {
            Some(bincode::serialize(&streak).expect("Error serializing jenkem streak"))
        } else {
            Some(bincode::serialize(&current_streak).expect("Error serializing jenkem streak"))
        }
    })
    .expect("Error updating jenkem streak");
}

pub fn get_jenkem_streak(db: &sled::Db) -> i32 {
    match db
        .get("jenkem_streak")
        .expect("Error getting jenkem streak")
    {
        Some(bytes) => bincode::deserialize(&bytes).expect("Error deserializing jenkem streak"),
        None => 0,
    }
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
    previous_holder: u64,
    huff_count: i32,
}

impl JenkemModel {
    pub fn new(current_holder: u64) -> JenkemModel {
        JenkemModel {
            current_holder,
            previous_holder: 0,
            huff_count: 0,
        }
    }

    pub fn pass(&mut self, recipient: u64) {
        self.huff_count += 1;
        self.previous_holder = self.current_holder;
        self.current_holder = recipient;
    }

    pub fn reject(&mut self) {
        self.current_holder = self.previous_holder;
        self.previous_holder = 0;
    }

    pub fn huff_count(&self) -> i32 {
        self.huff_count
    }

    pub fn current_holder(&self) -> u64 {
        self.current_holder
    }

    pub fn previous_holder(&self) -> u64 {
        self.previous_holder
    }
}
