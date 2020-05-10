use serenity::{model::channel::Message, prelude::Context};

pub fn kick(ctx: Context, msg: &Message) {
    msg.channel_id
        .say(&ctx.http, "Begone!")
        .expect("Error sending message");

    msg.guild_id
        .unwrap()
        .kick(&ctx.http, &msg.author)
        .expect("Error kicking user");
}
