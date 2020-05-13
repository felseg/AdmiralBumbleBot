use {
    super::common,
    crate::logging,
    serenity::{
        model::{channel::Message, id::UserId},
        prelude::Context,
    },
};

const BAN_DELETE_DAYS: u8 = 0;

pub enum Punishment {
    Kick,
    Ban,
    Mute,
    Unmute,
}

pub fn punish(
    ctx: &Context,
    msg: &Message,
    target: &str,
    args: &str,
    punishment_type: &Punishment,
) {
    let guild_id = msg.guild_id.expect("Error getting guild ID");
    let author = &msg.author;

    if common::confirm_admin(&ctx, &author, guild_id) || d20::roll_dice("2d20").unwrap().total >= 39
    {
        match punishment_type {
            Punishment::Kick => {
                if let Err(e) = msg
                    .guild_id
                    .unwrap()
                    .kick(&ctx.http, UserId(target.parse().unwrap()))
                {
                    eprintln!("Error kicking member {}: {}", &target, e);
                }

                let log_text = format!(
                    "👊 <@!{}> was kicked by <@!{}>:\n` ┗ Reason: {}`",
                    target, author.id, args
                );

                if let Err(e) = msg.channel_id.say(&ctx.http, &log_text) {
                    eprintln!("Error sending message: {}", e);
                }
                logging::log(ctx, &log_text);
            }
            Punishment::Ban => {
                if let Err(e) = msg.guild_id.unwrap().ban(
                    &ctx.http,
                    UserId(target.parse().unwrap()),
                    &(BAN_DELETE_DAYS, args),
                ) {
                    eprintln!("Error banning member {}: {}", &target, e);
                }

                let log_text = format!(
                    "🚫 <@!{}> was banned by <@!{}>:\n` ┗ Reason: {}`",
                    target, author.id, args
                );

                if let Err(e) = msg.channel_id.say(&ctx.http, &log_text) {
                    eprintln!("Error sending message: {}", e);
                }
                logging::log(ctx, &log_text);
            }
            Punishment::Mute => {
                let mut member = ctx
                    .http
                    .get_member(*guild_id.as_u64(), target.parse().unwrap())
                    .expect("Error getting user");

                if let Err(e) = member.add_role(&ctx.http, get_env!("ABB_MUTE_ROLE", u64)) {
                    eprintln!("Error muting user: {}", e);
                }

                let log_text = format!(
                    "🤐 <@!{}> was muted by <@!{}>:\n` ┗ Reason: {}`",
                    target, author.id, args
                );

                if let Err(e) = msg.channel_id.say(&ctx.http, &log_text) {
                    eprintln!("Error sending message: {}", e);
                }
                logging::log(ctx, &log_text);
            }
            Punishment::Unmute => {
                let mut member = ctx
                    .http
                    .get_member(*guild_id.as_u64(), target.parse().unwrap())
                    .expect("Error getting user");

                if let Err(e) = member.remove_role(&ctx.http, get_env!("ABB_MUTE_ROLE", u64)) {
                    eprintln!("Error muting user: {}", e);
                }

                let log_text = format!("🤐 <@!{}> was unmuted by <@!{}>", target, author.id);

                if let Err(e) = msg.channel_id.say(&ctx.http, &log_text) {
                    eprintln!("Error sending message: {}", e);
                }
                logging::log(ctx, &log_text);
            }
        };
    }
}
