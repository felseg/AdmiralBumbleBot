use crate::commands;
use crate::logging;

use serenity::client::EventHandler;

use crate::variables::Variables;
use serenity::{
    model::{
        channel::Message,
        event::MessageUpdateEvent,
        guild::Member,
        id::{ChannelId, GuildId, MessageId},
        user::User,
    },
    prelude::*,
};

pub struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        commands::execute(ctx, msg);
    }

    fn message_update(
        &self,
        ctx: Context,
        old_if_available: Option<Message>,
        new: Option<Message>,
        _event: MessageUpdateEvent,
    ) {
        if let Some(msg) = old_if_available {
            let new_content = new.unwrap().content;
            if msg.content == new_content {
                //Embeds register as identical edits for some reason
                return;
            }

            let old_stripped = &msg.content.clone().replace("`", "");
            let new_stripped = &new_content.replace("`", "");

            logging::log(
                ctx,
                format!(
                    "✏️ Message edited by `{}#{}` in <#{}>:\n` ┣ Original: {}`\n` ┗ Edited:   {}`",
                    msg.author.name,
                    msg.author.discriminator,
                    msg.channel_id,
                    old_stripped,
                    new_stripped
                )
                .as_ref(),
            );
        }
    }

    fn message_delete(&self, ctx: Context, channel_id: ChannelId, message_id: MessageId) {
        let deleted_message = ctx.cache.read().message(channel_id, message_id);
        if let Some(message) = deleted_message {
            let stripped_message = &message.content.clone().replace("`", "");

            logging::log(
                ctx,
                format!(
                    "🗑 Message deleted in <#{}>: `{}#{}: {}`",
                    channel_id, message.author.name, message.author.discriminator, stripped_message
                )
                .as_str(),
            );
        }
    }

    fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, mut new_member: Member) {
        let join_roles = vec![Variables::join_role_1(), Variables::join_role_2()];

        new_member
            .add_role(
                &ctx.http,
                join_roles[d20::roll_dice("1d2").unwrap().total as usize - 1],
            )
            .expect("Error roling new user");

        logging::log(
            ctx,
            format!("📥 User joined: `{}`", new_member.distinct()).as_str(),
        );
    }

    fn guild_member_removal(
        &self,
        ctx: Context,
        _guild: GuildId,
        user: User,
        _member_data_if_available: Option<Member>,
    ) {
        logging::log(
            ctx,
            format!("📤 User left: `{}#{}`", user.name, user.discriminator).as_str(),
        );
    }
}
