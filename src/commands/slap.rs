use serenity::{
    model::{channel::Message, id::UserId},
    prelude::Context,
};

pub fn slap(ctx: &Context, msg: &Message, target: &str, args: &str) {
    let slapper = &msg.author.name;
    let slappee = UserId(target.parse().expect("Error parsing target"))
        .to_user(&ctx.http)
        .unwrap()
        .name;

    let message_text = if args
        .to_ascii_uppercase()
        .starts_with(&['A', 'E', 'I', 'O', 'U'][..])
    {
        format!(
            "*{} slaps {} in the face with an {}!*",
            slapper, slappee, args
        )
    } else {
        format!(
            "*{} slaps {} in the face with a {}!*",
            slapper, slappee, args
        )
    };

    msg.channel_id
        .say(&ctx.http, message_text)
        .expect("Error sending message");
}
