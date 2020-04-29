use crate::variables::Variables;
use serenity::client;
use serenity::model::id::ChannelId;

pub fn log(ctx: client::Context, message: &str) {
    if let Err(e) = ChannelId(Variables::log_channel()).say(ctx.http, message) {
        eprintln!("Error sending message: {}", e);
    }
}
