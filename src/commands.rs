use {
    regex::Regex,
    serenity::{model::channel::Message, prelude::Context},
};

mod announcement;
mod bee_sting;
mod buzz;
mod clean;
mod common;
mod get_message_data;
mod give_admin;
mod help;
mod punish;
mod slap;

pub fn execute(ctx: &Context, msg: &Message, db: &sled::Db) {
    if !msg.content.starts_with('$') {
        return;
    }

    let (command, target, args) = match parse_command(&msg.content.as_str()) {
        Some(result) => result,
        None => return,
    };

    if d20::roll_dice("1d20").unwrap().total == 20
        && *msg.channel_id.as_u64() != get_env!("ABB_BOT_TEST_CHANNEL", u64)
    {
        bee_sting::bee_sting(ctx, &msg, &command, &target, &args);
        return;
    }

    match command.as_str() {
        "$help" => help::help(&ctx, &msg),
        "$buzz" => buzz::buzz(&ctx, &msg),
        "$kick" => punish::punish(ctx, &msg, &target, &args, &punish::Punishment::Kick),
        "$ban" => punish::punish(ctx, &msg, &target, &args, &punish::Punishment::Ban),
        "$mute" => punish::punish(ctx, &msg, &target, &args, &punish::Punishment::Mute),
        "$unmute" => punish::punish(ctx, &msg, &target, &args, &punish::Punishment::Unmute),
        "$announcement" => announcement::announcement(&ctx, &msg),
        "$giveAdmin" => give_admin::give_admin(ctx, &msg),
        "$clean" => clean::clean(ctx, &msg, &args),
        "$getMessageData" => get_message_data::get_message_data(&ctx, &msg, &target, &db),
        "$slap" => slap::slap(&ctx, &msg, &target, &args),
        _ => {}
    };
}

fn parse_command(text: &str) -> Option<(String, String, String)> {
    let regexes = vec![
        Regex::new(r"(?P<command>^\$\w+) <@!(?P<target>\d+)> (?P<args>.*)").unwrap(),
        Regex::new(r"(?P<command>^\$\w+) <@!(?P<target>\d+)>").unwrap(),
        Regex::new(r"(?P<command>^\$\w+) <@(?P<target>\d+)> (?P<args>.*)").unwrap(),
        Regex::new(r"(?P<command>^\$\w+) <@(?P<target>\d+)>").unwrap(),
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
