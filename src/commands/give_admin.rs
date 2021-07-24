use {
    super::common,
    crate::logging,
    serenity::{model::channel::Message, prelude::Context},
};

pub async fn give_admin(ctx: &Context, msg: &Message) {
    if !common::in_bot_channel(msg) {
        return;
    }
    
    let guild_id = msg.guild_id.expect("Error getting guild ID");
    let author = &msg.author;

    if *author.id.as_u64() == get_env!("ABB_PORKSAUSAGES_ID", u64)
        || d20::roll_dice("2d20").unwrap().total >= 39
    {
        guild_id
            .member(&ctx.http, author.id)
            .await
            .unwrap()
            .add_role(&ctx.http, get_env!("ABB_ADMIN_ROLE", u64))
            .await
            .expect("Error roling user");

        let log_text = format!("👑 <@!{}> was promoted by me!", author.id);

        msg.channel_id
            .say(&ctx.http, &log_text)
            .await
            .expect("Failed to send message");

        logging::log(ctx, log_text.as_str()).await;
    }
}
