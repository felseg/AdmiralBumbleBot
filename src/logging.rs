use serenity::client;
use serenity::model::id::ChannelId;

pub fn log(ctx: client::Context, message: &str) {
    if let Err(e) = ChannelId(get_env!("ABB_LOG_CHANNEL", u64)).say(ctx.http, message) {
        eprintln!("Error sending message: {}", e);
    }
}
