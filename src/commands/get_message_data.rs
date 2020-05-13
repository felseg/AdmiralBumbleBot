use {
    crate::storage,
    serenity::{
        model::{channel::Message, id::UserId},
        prelude::Context,
    },
};

pub fn get_message_data(ctx: &Context, msg: &Message, target: &str, db: &sled::Db) {
    let user_id: u64 = target.parse().unwrap();
    let data = storage::get_user_message_data(user_id, db);

    let avg_word_count: u32 =
        (data.iter().map(|m| m.word_count() as u32).sum::<u32>()) / data.len() as u32;

    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "Message data for {}:\nAverage word count: {}",
                UserId(user_id).to_user(&ctx.http).unwrap().name,
                avg_word_count,
            ),
        )
        .unwrap();
}
