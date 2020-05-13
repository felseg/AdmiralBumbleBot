use serenity::{
    model::{channel::Message, id::UserId},
    prelude::Context,
};

pub fn slap(ctx: &Context, msg: &Message, target: &str, args: &str) {
    let slapper = &msg.author.name;
    println!("{}", target);
    let slappee = UserId(
        target
            .parse()
            .expect("I cant parse because I'm a stupid robot"),
    )
    .to_user(&ctx.http)
    .unwrap()
    .name;

    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "*{} slaps {} in the face with a {}!*",
                slapper, slappee, args,
            ),
        )
        .expect("Error sending message");
}
