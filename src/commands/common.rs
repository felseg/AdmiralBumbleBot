use serenity::{
    model::{
        guild::Member,
        id::{GuildId, RoleId},
        prelude::User,
    },
    prelude::Context,
};

pub enum Punishment {
    Kick,
    Ban,
    Mute,
    Unmute,
}

pub fn random_user(ctx: &Context, guild_id: &GuildId) -> Member {
    let member_count = guild_id
        .to_guild_cached(&ctx.cache)
        .unwrap()
        .read()
        .member_count;

    let members: Vec<Member> = guild_id
        .members(&ctx.http, Some(member_count), None)
        .unwrap();

    members[(d20::roll_dice(format!("1d{}", members.len()).as_str())
        .unwrap()
        .total) as usize
        - 1]
    .clone()
}

pub fn confirm_admin(ctx: &Context, user: &User, guild: GuildId) -> bool {
    match user.has_role(&ctx.http, guild, RoleId(get_env!("ABB_ADMIN_ROLE", u64))) {
        Ok(b) => {
            if b || user.id == get_env!("ABB_USER_ID", u64) {
                //If command user has Admin role or is AdmiralBumbleBee himself
                true
            } else {
                false
            }
        }
        Err(e) => {
            eprintln!("Error authenticating user: {}", e);
            false
        }
    }
}
