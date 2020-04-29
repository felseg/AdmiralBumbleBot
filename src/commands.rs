use {
    crate::logging,
    crate::variables::Variables,
    d20,
    regex::Regex,
    serenity::{
        model::{
            channel::Message,
            id::{GuildId, RoleId, UserId},
            user::User,
        },
        prelude::*,
    },
};

pub fn execute(ctx: Context, msg: Message) {
    if !msg.content.starts_with("$") {
        return;
    }

    let (command, target, args) = match parse_command(&msg.content.as_str()) {
        Some(result) => result,
        None => return,
    };

    match command.as_str() {
        "$buzz" => buzz(ctx, &msg),
        "$kick" => kick(ctx, &msg, &target, &args),
        _ => {}
    };
}

fn buzz(ctx: Context, msg: &Message) {
    if let Err(e) = msg.channel_id.say(&ctx.http, "BUZZ!") {
        println!("Error sending message: {:?}", e);
    }
}

fn kick(ctx: Context, msg: &Message, target: &str, args: &str) {
    let guild_id = *&msg.guild_id.expect("Error getting guild ID");
    let author = &msg.author;

    if confirm_admin(&ctx, &author, guild_id) || d20::roll_dice("2d20").unwrap().total >= 39 {
        if let Err(e) = msg
            .guild_id
            .unwrap()
            .kick(&ctx.http, UserId(target.parse().unwrap()))
        {
            eprintln!("Error kicking member {}: {}", &target, e);
        }

        logging::log(
            ctx,
            format!(
                "👊 <@!{}> was kicked by <@!{}>:\n` ┗ Reason: {}`",
                target, author.id, args
            )
            .as_str(),
        );
    }
}

//TODO: Mute

//TODO: Ban

//TODO: Help

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
