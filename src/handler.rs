use {
    crate::{commands, logging, storage},
    serenity::{
        client::{Context, EventHandler},
        model::{
            channel::Message,
            event::MessageUpdateEvent,
            gateway::{Activity, Ready},
            guild::Member,
            id::{ChannelId, GuildId, MessageId},
            user::User,
        },
    },
    std::time,
};

pub struct Handler {
    pub storage: sled::Db,
}

impl EventHandler for Handler {
    fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, mut new_member: Member) {
        let join_roles: Vec<u64> = vec![
            get_env!("ABB_JOIN_ROLE_1", u64),
            get_env!("ABB_JOIN_ROLE_2", u64),
        ];

        new_member
            .add_role(
                &ctx.http,
                join_roles[d20::roll_dice("1d2").unwrap().total as usize - 1],
            )
            .expect("Error roling new user");

        logging::log(
            &ctx,
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
            &ctx,
            format!("📤 User left: `{}#{}`", user.name, user.discriminator).as_str(),
        );
    }

    fn message(&self, ctx: Context, msg: Message) {
        commands::execute(&ctx, &msg, &self.storage);

        let user_id = *msg.author.id.as_u64();
        let channel_id = *msg.channel_id.as_u64();
        let words: Vec<&str> = msg.content.split(' ').collect();
        let timestamp = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        storage::log_activity(
            user_id,
            channel_id,
            words.len() as u16,
            timestamp,
            &self.storage,
        );
    }

    fn message_delete(&self, ctx: Context, channel_id: ChannelId, message_id: MessageId) {
        let deleted_message = ctx.cache.read().message(channel_id, message_id);
        if let Some(message) = deleted_message {
            let stripped_message = message.content.replace("`", "");

            logging::log(
                &ctx,
                format!(
                    "🗑 Message deleted in <#{}>: `{}#{}: {}`",
                    channel_id, message.author.name, message.author.discriminator, stripped_message
                )
                .as_str(),
            );
        }
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

            let old_stripped = &msg.content.replace("`", "");
            let new_stripped = &new_content.replace("`", "");

            logging::log(
                &ctx,
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

    fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.set_activity(Activity::playing(
            "See my insides at https://git.io/JfW94 😘",
        ));
    }
}
