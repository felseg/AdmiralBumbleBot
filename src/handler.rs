use crate::commands;
use crate::variables::Variables;

use serenity::client::EventHandler;

use serenity::{
    model::{
        channel::Message,
        event::MessageUpdateEvent,
        id::{ChannelId, MessageId},
    },
    prelude::*,
};

pub struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        commands::execute(ctx, msg);
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
