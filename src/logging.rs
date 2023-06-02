use serenity::{model::id::ChannelId, prelude::Context};

pub async fn log(ctx: &Context, message: &str) {
    if let Err(e) = ChannelId(get_env!("ABB_LOG_CHANNEL", u64))
        .send_message(&ctx.http, |msg| {
            msg.content(message).allowed_mentions(|am| am.empty_parse())
        })
        .await
    {
        eprintln!("Error sending message: {}", e);
    }
}
