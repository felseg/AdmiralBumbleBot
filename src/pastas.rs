use serenity::{client::Context, model::channel::Message};

const OK_PASTA: &str = "\"Ok\"? Are you fucking kidding me? I spent a decent portion of my life writing all of that and your response to me is \"Ok\"? Are you so mentally handicapped that the only word you can comprehend is \"Ok\" - or are you just some fucking asshole who thinks that with such a short response, he can make a statement about how meaningless what was written was? Well, I'll have you know that what I wrote was NOT meaningless, in fact, I even had my written work proof-read by several professors of literature. Don't believe me? I doubt you would, and your response to this will probably be \"Ok\" once again. Do I give a fuck? No, does it look like I give even the slightest fuck about two fucking letters? I bet you took the time to type those two letters too, I bet you sat there and chuckled to yourself for 20 hearty seconds before pressing \"send\". You're so fucking pathetic. I'm honestly considering directing you to a psychiatrist, but I'm simply far too nice to do something like that. You, however, will go out of your way to make a fool out of someone by responding to a well-thought-out, intelligent, or humorous statement that probably took longer to write than you can last in bed with a chimpanzee. What do I have to say to you? Absolutely nothing. I couldn't be bothered to respond to such a worthless attempt at a response. Do you want \"Ok\" on your gravestone?";

pub async fn copypastas(ctx: &Context, msg: &Message) {
    if msg.channel_id.0 == get_env!("ABB_SHITPOST_CHANNEL", u64) {
        return;
    }

    if msg.content.to_ascii_lowercase() == OK_PASTA.to_ascii_lowercase()
        && msg.author.id.0 != get_env!("ABB_BOT_USER_ID", u64)
    {
        msg.channel_id
            .say(&ctx.http, "Ok")
            .await
            .expect("Error sending message");
    }

    if d20::roll_dice("1d20").unwrap().total > 10 {
        return;
    }

    if msg.content.to_ascii_lowercase() == "ok"
        && msg.author.id.0 != get_env!("ABB_BOT_USER_ID", u64)
    {
        msg.channel_id
            .say(&ctx.http, OK_PASTA)
            .await
            .expect("Error sending message");
    }
}
