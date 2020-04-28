use {
    regex::Regex,
    serenity::{model::channel::Message, prelude::*},
};

pub fn execute(ctx: Context, msg: Message) {
    if !msg.content.starts_with("$") {
        return;
    }

    let re = Regex::new(r"/(?<command>^\$\w+) (?<target><@\d+>) (?<rem>.*)/g").unwrap();
    let caps = re.captures(msg.content.as_str()).unwrap();

    match &caps["command"] {
        "$buzz" => buzz(ctx, msg),
        _ => {}
    };
}

fn buzz(ctx: Context, msg: Message) {
    if let Err(e) = msg.channel_id.say(&ctx.http, "BUZZ!") {
        println!("Error sending message: {:?}", e);
    }
}
