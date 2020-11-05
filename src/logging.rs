use serenity::{model::id::ChannelId, prelude::Context};

pub async fn log(ctx: &Context, message: &str) {
    if let Err(e) = ChannelId(get_env!("ABB_LOG_CHANNEL", u64))
        .say(&ctx.http, message)
        .await
    {
        eprintln!("Error sending message: {}", e);
    }
}
