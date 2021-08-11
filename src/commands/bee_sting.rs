use {
    num_derive::FromPrimitive,
    num_traits::FromPrimitive,
    serenity::{model::channel::Message, prelude::Context},
};

mod create_dumb_channel;
mod kick;
mod mute;

const NUMBER_OF_STINGS: u8 = 3;

pub async fn bee_sting(ctx: &Context, msg: &Message) {
    msg.channel_id
        .say(&ctx.http, "*Stings you*")
        .await
        .expect("Error sending message");
    let roll = roll();

    match FromPrimitive::from_u8(roll) {
        Some(Sting::CreateDumbChannel) => create_dumb_channel::create_dumb_channel(ctx, msg).await,
        Some(Sting::Kick) => kick::kick(ctx, msg).await,
        Some(Sting::Mute) => mute::mute(ctx, msg).await,
        None => {}
    }
}

fn roll() -> u8 {
    d20::roll_dice(format!("1d{}", NUMBER_OF_STINGS).as_str())
        .unwrap()
        .total as u8
}

#[derive(FromPrimitive)]
enum Sting {
    CreateDumbChannel = 1,
    Kick,
    Mute,
}
