use {
    regex::Regex,
    serenity::{
        model::{channel::Message, channel::ReactionType, id::EmojiId},
        prelude::Context,
    },
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

    if msg.content.to_ascii_lowercase() == "ok" {
        msg.channel_id.say(&ctx.http, 
            "\"Ok\"? Are you fucking kidding me? I spent a decent portion of my life writing all of that and your response to me is \"Ok\"? Are you so mentally handicapped that the only word you can comprehend is \"Ok\" - or are you just some fucking asshole who thinks that with such a short response, he can make a statement about how meaningless what was written was? Well, I'll have you know that what I wrote was NOT meaningless, in fact, I even had my written work proof-read by several professors of literature. Don't believe me? I doubt you would, and your response to this will probably be \"Ok\" once again. Do I give a fuck? No, does it look like I give even the slightest fuck about two fucking letters? I bet you took the time to type those two letters too, I bet you sat there and chuckled to yourself for 20 hearty seconds before pressing \"send\". You're so fucking pathetic. I'm honestly considering directing you to a psychiatrist, but I'm simply far too nice to do something like that. You, however, will go out of your way to make a fool out of someone by responding to a well-thought-out, intelligent, or humorous statement that probably took longer to write than you can last in bed with a chimpanzee. What do I have to say to you? Absolutely nothing. I couldn't be bothered to respond to such a worthless attempt at a response. Do you want \"Ok\" on your gravestone?"
        ).await
        .expect("Error sending message");
    }

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
