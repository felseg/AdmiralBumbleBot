use {
    super::pastas,
    regex::Regex,
    serenity::{
        model::{channel::Message, channel::ReactionType, id::EmojiId},
        prelude::Context,
    },
    std::collections::HashMap,
    url::form_urlencoded,
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

pub async fn execute(ctx: &Context, msg: &Message, db: &sled::Db) {
    sonic(&ctx, &msg).await;
    pastas::copypastas(&ctx, &msg).await;
    consciousness(&ctx, &msg).await;

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
        bee_sting::bee_sting(ctx, &msg, &command, &target, &args).await;
        return;
    }

    match command.as_str() {
        "$help" => help::help(&ctx, &msg).await,
        "$buzz" => buzz::buzz(&ctx, &msg).await,
        "$kick" => punish::punish(ctx, &msg, &target, &args, &punish::Punishment::Kick).await,
        "$ban" => punish::punish(ctx, &msg, &target, &args, &punish::Punishment::Ban).await,
        "$mute" => punish::punish(ctx, &msg, &target, &args, &punish::Punishment::Mute).await,
        "$unmute" => punish::punish(ctx, &msg, &target, &args, &punish::Punishment::Unmute).await,
        "$announcement" => announcement::announcement(&ctx, &msg).await,
        "$giveAdmin" => give_admin::give_admin(ctx, &msg).await,
        "$clean" => clean::clean(ctx, &msg, &args).await,
        "$getMessageData" => get_message_data::get_message_data(&ctx, &msg, &target, &db).await,
        "$slap" => slap::slap(&ctx, &msg, &target, &args).await,
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

async fn sonic(ctx: &Context, msg: &Message) {
    if msg.content.to_ascii_lowercase().contains("sonic")
        || msg.content.to_ascii_lowercase().contains("sanic")
    {
        msg.react(
            &ctx.http,
            ReactionType::Custom {
                id: EmojiId(724619044606574645),
                animated: false,
                name: Some(String::from("sonic-1")),
            },
        )
        .await
        .expect("I literally can't even");
    }
}

async fn consciousness(ctx: &Context, msg: &Message) {
    if msg
        .content
        .starts_with(&format!("<@!{}>", get_env!("ABB_BOT_USER_ID")))
        || msg
            .content
            .starts_with(&format!("<@{}>", get_env!("ABB_BOT_USER_ID")))
    //Wtf is this rustfmt
    {
        let content = msg.content.split_once('>').unwrap().1.trim();

        let request_url = form_urlencoded::Serializer::new(format!(
            "{}?key={}",
            get_env!("ABB_CLEVERBOT_URL"),
            get_env!("ABB_CLEVERBOT_API_KEY")
        ))
        .append_pair("input", content)
        .append_pair("cs", &get_env!("ABB_CLEVERBOT_STATE"))
        .finish();

        let response = reqwest::get(&request_url)
            .await
            .expect("Error making request to Cleverbot API")
            .json::<HashMap<String, String>>()
            .await
            .expect("Error deserializing Cleverbot response");

        let response_message = format!("<@{}> {}", msg.author.id.0, response["output"]);

        msg.channel_id
            .say(&ctx.http, &response_message)
            .await
            .expect("Error sending message");

        return;
    }
}
