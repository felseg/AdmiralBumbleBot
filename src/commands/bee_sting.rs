use {
    d20,
    num_derive::FromPrimitive,
    num_traits::FromPrimitive,
    serenity::{model::channel::Message, prelude::Context},
};

mod create_dumb_channel;
mod kick;
mod mute;

const NUMBER_OF_STINGS: u8 = 3;

pub fn bee_sting(ctx: Context, msg: &Message, _command: &str, _target: &str, _args: &str) {
    msg.channel_id
        .say(&ctx.http, "*Stings you*")
        .expect("Error sending message");
    let roll = roll();

    match FromPrimitive::from_u8(roll) {
        Some(Sting::CreateDumbChannel) => create_dumb_channel::create_dumb_channel(ctx, msg),
        Some(Sting::Kick) => kick::kick(ctx, msg),
        Some(Sting::Mute) => mute::mute(ctx, msg),
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

#[cfg(test)]
mod tests {}
