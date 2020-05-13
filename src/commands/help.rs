use serenity::{model::channel::Message, prelude::Context, utils::Color};

pub fn help(ctx: &Context, msg: &Message) {
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
                (
                    "$getMessageData `{target}`",
                    "Shows some info about the users posting habits. (UNFINISHED)",
                    true,
                ),
                (
                    "$slap `{target}` `{object}`",
                    "Slap somebody with an object of your choosing.",
                ),
            ]);
            e
        });
        m
    }) {
        eprintln!("Error displaying help: {}", e);
    }
}
