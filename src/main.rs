use std::env;

use serenity::{
    model::{
        channel::Message,
        event::MessageUpdateEvent,
        id::{ChannelId, MessageId},
    },
    prelude::*,
};

const CACHE_SIZE: usize = 100;

struct Handler;
struct Variables;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "$buzz" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "BUZZ!") {
                println!("Error sending message: {:?}", e);
            }
        }
        if msg.content.starts_with("$help") {
            if let Err(e) = msg.channel_id.say(&ctx.http, "no") {
                println!("Error sending message: {:?}", e);
            }
        }
    }

    fn message_update(
        &self,
        ctx: Context,
        old_if_available: Option<Message>,
        new: Option<Message>,
        _event: MessageUpdateEvent,
    ) {
        if let Some(msg) = old_if_available {
            if let Err(e) = ChannelId(Variables::log_channel()).say(
                ctx.http,
                format!(
                    "✏️ Message edited by `{}#{}` in <#{}>:\n` ┣ Original: {}`\n` ┗ Edited:   {}`",
                    msg.author.name,
                    msg.author.discriminator,
                    msg.channel_id,
                    msg.content,
                    new.unwrap().content
                ),
            ) {
                eprintln!("Error sending message: {}", e);
            }
        }
    }

    fn message_delete(&self, ctx: Context, channel_id: ChannelId, message_id: MessageId) {
        let deleted_message = ctx.cache.read().message(channel_id, message_id);
        if let Some(message) = deleted_message {
            if let Err(e) = ChannelId(Variables::log_channel()).say(
                ctx.http,
                format!(
                    "🗑 Message deleted in <#{}>: `{}#{}: {}`",
                    channel_id, message.author.name, message.author.discriminator, message.content
                ),
            ) {
                eprintln!("Error sending message: {}", e);
            }
        }
    }
}

impl Variables {
    fn log_channel() -> u64 {
        env::var("ABB_LOG_CHANNEL")
            .expect("Log channel not found")
            .parse()
            .expect("Error parsing log channel")
    }

    fn token() -> String {
        env::var("ABB_TOKEN").expect("Token not found")
    }
}

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
