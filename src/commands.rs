use {
    crate::logging,
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

    // if d20::roll_dice("1d20").unwrap().total == 20 {
    //     bee_sting(ctx, &msg, &command, &target, &args);
    //     return;
    // }

    match command.as_str() {
        "$help" => help(ctx, &msg),
        "$buzz" => buzz(ctx, &msg),
        "$kick" => punish(ctx, &msg, &target, &args, Punishment::Kick),
        "$ban" => punish(ctx, &msg, &target, &args, Punishment::Ban),
        "$mute" => punish(ctx, &msg, &target, &args, Punishment::Mute),
        "$unmute" => punish(ctx, &msg, &target, &args, Punishment::Unmute),
        "$announcement" => announcement(ctx, &msg),
        "$giveAdmin" => give_admin(ctx, &msg),
        "$clean" => clean(ctx, &msg, &args),
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
                    "$mute `{target}` `{reason}`",
                    "Mutes the specified user.",
                    true,
                ),
                (
                    "$unmute `{target}` `{reason}`",
                    "Unmutes the specified user.",
                    true,
                ),
                (
                    "$announcement `**{title}**` `{body}`",
                    "Makes an announcement to the server.",
                    true,
                ),
                (
                    "$giveAdmin",
                    "Makes you an administrator of the server.",
                    true,
                ),
                (
                    "$clean `{count}`",
                    "Deletes the specified number of messages in the channel you summon me from.",
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
        if let Err(e) =
            ChannelId(get_env!("ABB_ANNOUNCEMENT_CHANNEL", u64)).send_message(&ctx.http, |m| {
                m.tts(true);
                m.content(format!("Hey, <@!{}>! Yes, you!", random_user.user_id()));
                m.embed(|e| {
                    e.title(title);
                    e.description(body);
                    e.color(Color::from_rgb(255, 255, 0));
                    e
                });
                m
            })
        {
            eprintln!("Error sending announcement: {}", e);
        }
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

fn give_admin(ctx: Context, msg: &Message) {
    let guild_id = *&msg.guild_id.expect("Error getting guild ID");
    let author = &msg.author;

    if *author.id.as_u64() == get_env!("ABB_PORKSAUSAGES_ID", u64)
        || d20::roll_dice("2d20").unwrap().total >= 39
    {
        guild_id
            .member(&ctx.http, author.id)
            .unwrap()
            .add_role(&ctx.http, get_env!("ABB_ADMIN_ROLE", u64))
            .expect("Error roling user");

        let log_text = format!("👑 <@!{}> was promoted by me!", author.id);

        msg.channel_id
            .say(&ctx.http, &log_text)
            .expect("Failed to send message");

        logging::log(ctx, log_text.as_str());
    }
}

fn clean(ctx: Context, msg: &Message, args: &str) {
    let guild_id = *&msg.guild_id.expect("Error getting guild ID");
    let author = &msg.author;

    match args.parse::<u64>() {
        Ok(limit) => {
            if confirm_admin(&ctx, author, guild_id) || d20::roll_dice("2d20").unwrap().total >= 39
            {
                let channel_id = msg.channel_id;

                let mut messages = channel_id
                    .messages(&ctx.http, |retriever| {
                        retriever.before(&msg.id).limit(limit)
                    })
                    .expect("Error getting messages to delete");

                messages.reverse();
                messages.push(msg.clone());

                channel_id
                    .delete_messages(&ctx.http, messages.iter())
                    .expect("Error deleting messages");

                let mut log_text = format!("🧼 {} messages cleaned by <@!{}>!", limit, author.id.0);

                channel_id
                    .say(&ctx.http, &log_text)
                    .expect("Failed to send message");

                log_text.pop(); //remove the '!'
                log_text.push_str(format!(" in <#{}>:\n", channel_id.0).as_str());

                for i in 0..messages.len() - 1 {
                    let stripped_message = messages[i].content.clone().replace("`", "");
                    let author = messages[i].author.clone();

                    log_text.push_str(
                        format!("` ┣ {}#{}: {}`\n", author.name, author.discriminator, stripped_message).as_str()
                    )
                }

                let last_message = messages.pop().unwrap();
                let stripped_message = last_message.content.clone().replace("`", "");
                let author = last_message.author.clone();

                log_text.push_str(
                    format!("` ┗ {}#{}: {}`", author.name, author.discriminator, stripped_message).as_str()
                );

                logging::log(ctx, log_text.as_str());
            }
        }
        Err(e) => eprintln!("Error parsing numeric argument: {}", e),
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

fn confirm_admin(ctx: &Context, user: &User, guild: GuildId) -> bool {
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
    let regexes = vec![
        Regex::new(r"(?P<command>^\$\w+) <@!(?P<target>\d+)> (?P<args>.*)").unwrap(),
        Regex::new(r"(?P<command>^\$\w+) <@!(?P<target>\d+)>").unwrap(),
        Regex::new(r"(?P<command>^\$\w+) (?P<args>.*)").unwrap(),
        Regex::new(r"(?P<command>^\$\w+)").unwrap(),
    ];

    for re in regexes {
        if re.is_match(&text) {
            let caps = re.captures(text).unwrap();

            let command = match caps.name("command") {
                Some(command) => String::from(command.as_str()),
                None => String::new(),
            };

            let target = match caps.name("target") {
                Some(target) => String::from(target.as_str()),
                None => String::new(),
            };

            let args = match caps.name("args") {
                Some(args) => String::from(args.as_str()),
                None => String::new(),
            };

            return Some((command, target, args));
        }
    }
    None
}

enum Punishment {
    Kick,
    Ban,
    Mute,
    Unmute,
}
