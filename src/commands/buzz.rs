use serenity::{model::channel::Message, prelude::Context};

pub fn buzz(ctx: &Context, msg: &Message) {
    if let Err(e) = msg.channel_id.say(&ctx.http, "BUZZ!") {
        println!("Error sending message: {:?}", e);
    }
}
