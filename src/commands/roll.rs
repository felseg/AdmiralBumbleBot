use serenity::{model::channel::Message, prelude::Context};

pub async fn roll(ctx: &Context, msg: &Message, args: &str) {
    let result = match d20::roll_dice(args) {
        Ok(result) => result.total,
        Err(_) => 0,
    };

    if result == 0 {
        msg.channel_id
            .say(&ctx.http, "Error in dice format!")
            .await
            .expect("Error sending message");

        return;
    }

    msg.channel_id
        .say(&ctx.http, format!("{}!", result))
        .await
        .expect("Error sending message");
}
