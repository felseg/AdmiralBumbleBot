use {
    d20,
    ron::de::from_reader,
    serenity::{model::channel::Message, prelude::Context},
    std::{collections::HashMap, fs::File},
};

pub fn create_dumb_channel(ctx: Context, msg: &Message) {
    let (chan_name, chan_description) = match get_random_channel() {
        Some(res) => res,
        None => return,
    };

    msg.channel_id
        .say(&ctx.http, "Creating a fun new channel!")
        .expect("Error sending message");

    msg.guild_id
        .expect("Error getting guild ID")
        .create_channel(ctx.http, |ch| {
            ch.name(chan_name);
            ch.topic(chan_description);
            ch.category(get_env!("ABB_MAIN_CHANNEL_CATEGORY", u64));
            ch
        })
        .expect("Error creating channel");
}

fn get_random_channel() -> Option<(String, String)> {
    let f = File::open("dumb_channels.ron").expect("Error opening dumb channel list");

    let channel_names: HashMap<String, String> = match from_reader(f) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error reading dumb_channels.ron: {}", e);
            return None;
        }
    };

    let names: Vec<String> = channel_names.keys().cloned().collect();

    let roll = d20::roll_dice(format!("1d{}", names.len()).as_str())
        .unwrap()
        .total;

    Some((
        names[roll as usize].clone(),
        channel_names[names[roll as usize].as_str()].clone(),
    ))
}

#[cfg(test)]
mod tests {
    use {
        ron::de::from_reader,
        std::{collections::HashMap, fs::File},
    };

    #[test]
    fn get_channel_names() {
        let f = File::open("dumb_channels.ron").expect("Error opening dumb channel list");
        let channel_names: HashMap<String, String> = match from_reader(f) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error reading dumb_channels.ron: {}", e);
                HashMap::new()
            }
        };

        assert!(!channel_names.is_empty());
    }
}
