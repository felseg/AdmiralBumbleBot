use serenity::{
    model::{
        channel::Message,
        guild::Member,
        id::{GuildId, RoleId},
        prelude::User,
    },
    prelude::Context,
};

use rand::Rng;

pub async fn random_user(ctx: &Context, guild_id: &GuildId) -> Member {
    let member_count = guild_id
        .to_guild_cached(&ctx.cache)
        .await
        .unwrap()
        .member_count;

    let members: Vec<Member> = guild_id
        .members(&ctx.http, Some(member_count), None)
        .await
        .unwrap(); //For some reason not supplying a limit makes it return a single member every time

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0, members.len());

    members[random_index].clone()
}

pub async fn confirm_admin(ctx: &Context, user: &User, guild: GuildId) -> bool {
    match user
        .has_role(&ctx.http, guild, RoleId(get_env!("ABB_ADMIN_ROLE", u64)))
        .await
    {
        Ok(b) => b || user.id == get_env!("ABB_USER_ID", u64),
        Err(e) => {
            eprintln!("Error authenticating user: {}", e);
            false
        }
    }
}

pub fn in_bot_channel(msg: &Message) -> bool {
    if msg.channel_id.0 == get_env!("ABB_BOT_CHANNEL", u64)
        || msg.channel_id.0 == get_env!("ABB_BOT_TEST_CHANNEL", u64)
    {
        return true;
    }
    false
}
