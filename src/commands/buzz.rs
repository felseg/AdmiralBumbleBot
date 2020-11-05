use serenity::{model::channel::Message, prelude::Context};

pub async fn buzz(ctx: &Context, msg: &Message) {
    if let Err(e) = msg.channel_id.say(&ctx.http, "BUZZ!").await {
        println!("Error sending message: {:?}", e);
    }
}
