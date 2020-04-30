use {
    crate::logging,
    crate::variables::Variables,
    d20,
    regex::Regex,
    serenity::{
        model::{
            channel::Message,
            guild::Member,
            id::{ChannelId, GuildId, RoleId, UserId},
            user::User,
        },
        prelude::*,
        utils::Color,
    },
};

const BAN_DELETE_DAYS: u8 = 0;

pub fn execute(ctx: Context, msg: Message) {
    if !msg.content.starts_with("$") {
        return;
    }

    let (command, target, args) = match parse_command(&msg.content.as_str()) {
        Some(result) => result,
        None => return,
    };

    match command.as_str() {
        "$help" => help(ctx, &msg),
        "$buzz" => buzz(ctx, &msg),
        "$kick" => punish(ctx, &msg, &target, &args, Punishment::Kick),
        "$ban" => punish(ctx, &msg, &target, &args, Punishment::Ban),
        "$announcement" => announcement(ctx, &msg),
        _ => {}
    };
}

fn help(ctx: Context, msg: &Message) {
    if let Err(e) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Help - Command List");
            e.color(Color::from_rgb(255, 255, 0));
            e.fields(vec![
                ("$help", "Show this again.", true),
                ("$buzz", "BUZZ!", true),
                (
                    "$kick `{target}` `{reason}`",
                    "Kicks the specified user.",
                    true,
                ),
                (
                    "$ban `{target}` `{reason}`",
                    "Bans the specified user.",
                    true,
                ),
                (
                    "$announcement `**{title}**` `{body}`",
                    "Makes an announcement to the server.",
                    true,
                ),
            ]);
            e
        });
        m
    }) {
        eprintln!("Error displaying help: {}", e);
    }
}

fn buzz(ctx: Context, msg: &Message) {
    if let Err(e) = msg.channel_id.say(&ctx.http, "BUZZ!") {
        println!("Error sending message: {:?}", e);
    }
}

fn announcement(ctx: Context, msg: &Message) {
    let guild_id = *&msg.guild_id.expect("Error getting guild ID");
    let author = &msg.author;

    let (title, body) = match parse_announcement_message(msg.content.as_str()) {
        Some(some) => some,
        None => return,
    };

    let random_user = random_user(&ctx, &guild_id);

    if confirm_admin(&ctx, &author, guild_id) || d20::roll_dice("2d20").unwrap().total >= 39 {
        /* Obnoxious embed with random ping */
        if let Err(e) = ChannelId(Variables::announcement_channel()).send_message(&ctx.http, |m| {
            m.tts(true);
            m.content(format!("Hey, <@!{}>! Yes, you!", random_user.user_id()));
            m.embed(|e| {
                e.title(title);
                e.description(body);
                e.color(Color::from_rgb(255, 255, 0));
                e
            });
            m
        }) {
            eprintln!("Error sending announcement: {}", e);
        }
    }
}

fn parse_announcement_message(message: &str) -> Option<(String, String)> {
    let re = Regex::new(r"(\*\*(?P<title>.*)\*\* (?P<body>.*))").unwrap();

    if !re.is_match(&message) {
        return None;
    }

    let caps = re.captures(message).unwrap();

    let (title, body) = {
        (
            caps.name("title")
                .expect("Error parsing announcement title")
                .as_str(),
            caps.name("body")
                .expect("Error parsing announcement body")
                .as_str(),
        )
    };

    Some((String::from(title), String::from(body)))
}

fn parse_command(text: &str) -> Option<(String, String, String)> {
    let re =
        Regex::new(r"(?P<arg_command>^\$\w+) <@!(?P<target>\d+)> (?P<args>.*)|(?P<command>^\$\w+)")
            .unwrap();

    if !re.is_match(&text) {
        return None;
    }

    let caps = re.captures(text).unwrap();

    match &caps.name("command") {
        Some(command) => Some((String::from(command.as_str()), String::new(), String::new())),
        None => Some((
            String::from(
                caps.name("arg_command")
                    .expect("Error parsing arg_command")
                    .as_str(),
            ),
            String::from(caps.name("target").expect("Error parsing target").as_str()),
            String::from(caps.name("args").expect("Error parsing args").as_str()),
        )),
    }
}

fn confirm_admin(ctx: &Context, user: &User, guild: GuildId) -> bool {
    match user.has_role(&ctx.http, guild, RoleId(Variables::admin_role())) {
        Ok(b) => {
            if b || user.id == Variables::abb_user_id() {
                //If command user has Admin role or is AdmiralBumbleBee himself
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

fn punish(ctx: Context, msg: &Message, target: &str, args: &str, punishment_type: Punishment) {
    let guild_id = *&msg.guild_id.expect("Error getting guild ID");
    let author = &msg.author;

    if confirm_admin(&ctx, &author, guild_id) || d20::roll_dice("2d20").unwrap().total >= 39 {
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
            _ => {}
        };
    }
}

fn random_user(ctx: &Context, guild_id: &GuildId) -> Member {
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

enum Punishment {
    Kick,
    Ban,
    _Mute, //TODO
}
