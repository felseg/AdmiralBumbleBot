use serenity::{
    model::{
        guild::Member,
        id::{GuildId, RoleId},
        prelude::User,
    },
    prelude::Context,
};

pub async fn random_user(ctx: &Context, guild_id: &GuildId) -> Member {
    let members: Vec<Member> = guild_id.members(&ctx.http, None, None).await.unwrap();

    members[(d20::roll_dice(format!("1d{}", members.len()).as_str())
        .unwrap()
        .total) as usize
        - 1]
    .clone()
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
