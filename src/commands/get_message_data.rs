use {
    super::common,
    crate::storage,
    crate::storage::MessageModel,
    serenity::{
        model::{channel::Message, id::UserId},
        prelude::Context,
    },
    std::collections::HashMap,
};

pub async fn get_message_data(ctx: &Context, msg: &Message, target: &str, db: &sled::Db) {
    if !common::in_bot_channel(msg) {
        return;
    }

    let user_id: u64 = target.parse().unwrap();

    let username = UserId(user_id).to_user(&ctx.http).await.unwrap().name;

    let data = storage::get_user_message_data(user_id, db);

    if data.is_empty() {
        msg.channel_id
            .say(&ctx.http, format!("No post data found for {}!", username))
            .await
            .unwrap();
        return;
    }

    let avg_word_count: u32 =
        (data.iter().map(|m| m.word_count() as u32).sum::<u32>()) / data.len() as u32;
    let total_posts: u32 = data.len() as u32;
    let favorite_channel = calculate_favourite_channel(&data);

    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "**Message data for {}:**\n  • Average word count: {}\n  • Total posts: {}\n  • Favorite channel: <#{}>",
                username,
                avg_word_count,
                total_posts,
                favorite_channel
            ),
        )
        .await
        .unwrap();
}

fn calculate_favourite_channel(data: &[MessageModel]) -> u64 {
    let mut data = data.to_vec();
    let mut stats: HashMap<u64, u32> = HashMap::new();
    let mut current_channel: u64 = data[0].channel_id();

    while !data.is_empty() {
        stats.insert(
            current_channel,
            data.drain_filter(|m| m.channel_id() == current_channel)
                .map(|_| 1)
                .sum(),
        );
        if !data.is_empty() {
            current_channel = data[0].channel_id();
        }
    }

    let mut highest: (u64, u32) = (0, 0);
    for count in stats {
        if count.1 > highest.1 {
            highest = count;
        }
    }

    highest.0
}
