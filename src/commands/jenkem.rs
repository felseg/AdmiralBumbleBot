use {
    crate::storage,
    serenity::{
        model::{channel::Message, id::UserId},
        prelude::Context,
    },
};

pub async fn pass_jenkem(ctx: &Context, msg: &Message, target: &str, db: &sled::Db) {
    let author = &msg.author;
    let recipient = UserId(target.parse().expect("Error parsing target"));
    let current_holder = storage::get_jenkem_holder(db);

    if current_holder != author.id.0 {
        msg.channel_id
            .say(&ctx.http, "You do not have the jenkem!")
            .await
            .expect("Error sending message");
        return;
    }

    let huff_count = storage::pass_jenkem(recipient.0, db);

    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "{} passed the jenkem to {}! The jenkem has been huffed {} times.",
                author.name,
                recipient
                    .to_user(&ctx.http)
                    .await
                    .expect("Error getting recipient")
                    .name,
                huff_count
            ),
        )
        .await
        .expect("Error sending message");
}

pub async fn brew_jenkem(ctx: &Context, msg: &Message, db: &sled::Db) {
    let author_name = &msg.author.name;
    let author_id = msg.author.id.0;
    storage::init_jenkem(author_id, db);

    msg.channel_id
        .say(
            &ctx.http,
            format!("{} brewed a new batch of jenkem!", author_name),
        )
        .await
        .expect("Error sending message");
}
