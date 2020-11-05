use serenity::{model::channel::Message, prelude::Context};

pub async fn kick(ctx: &Context, msg: &Message) {
    msg.channel_id
        .say(&ctx.http, "Begone!")
        .await
        .expect("Error sending message");

    msg.guild_id
        .unwrap()
        .kick(&ctx.http, &msg.author)
        .await
        .expect("Error kicking user");
}
