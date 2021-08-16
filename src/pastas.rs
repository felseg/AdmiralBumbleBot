use serenity::{client::Context, model::channel::Message};

const OK_PASTA: &str = "\"Ok\"? Are you fucking kidding me? I spent a decent portion of my life writing all of that and your response to me is \"Ok\"? Are you so mentally handicapped that the only word you can comprehend is \"Ok\" - or are you just some fucking asshole who thinks that with such a short response, he can make a statement about how meaningless what was written was? Well, I'll have you know that what I wrote was NOT meaningless, in fact, I even had my written work proof-read by several professors of literature. Don't believe me? I doubt you would, and your response to this will probably be \"Ok\" once again. Do I give a fuck? No, does it look like I give even the slightest fuck about two fucking letters? I bet you took the time to type those two letters too, I bet you sat there and chuckled to yourself for 20 hearty seconds before pressing \"send\". You're so fucking pathetic. I'm honestly considering directing you to a psychiatrist, but I'm simply far too nice to do something like that. You, however, will go out of your way to make a fool out of someone by responding to a well-thought-out, intelligent, or humorous statement that probably took longer to write than you can last in bed with a chimpanzee. What do I have to say to you? Absolutely nothing. I couldn't be bothered to respond to such a worthless attempt at a response. Do you want \"Ok\" on your gravestone?";
const BYE_PASTA: &str = "Bye. I'm leaving the server because I'm bored. Despite talking to all of you for a while I don't care about any of you at all. I lied. I'm not 14 I'm 13. And a girl. I can probably draw better than yall as well. Just sayin. Cya.";
const STONE_PASTA: &str = "🗿 is the worst emoji. It's horrendous and ugly. I hate it. The point of emojis is to show emotions, but what emotion does this show? Do you just wake up in the morning and think \"wow, I really feel like a massive fucking stone today\"? It's useless. I hate it. It just provokes a deep rooted anger within me whenever I see it. I want to drive on over to the fucking emoji headquarters and kill it. If this was the emoji movie I'd push it off a fucking cliff. People just comment 🗿 as if it's funny. It's not. 🗿 deserves to die. He deserves to have his smug little stone face smashed in with a hammer. Oh wow, it's a stone head, how fucking hilarious, I'll use it in every comment I post. NO. STOP IT. It deserves to burn in hell. Why is it so goddamn smug. You're a fucking stone, you have no life goals, you will never accomplish anything in life apart from pissing me off. When you die no one will mourn. I hope you die.";
const BASED_PASTA: &str = "Based? Based on what? In your dick? Please shut the fuck up and use words properly you fuckin troglodyte, do you think God gave us a freedom of speech just to spew random words that have no meaning that doesn't even correllate to the topic of the conversation? Like please you always complain about why no one talks to you or no one expresses their opinions on you because you're always spewing random shit like poggers based cringe and when you try to explain what it is and you just say that it's funny like what? What the fuck is funny about that do you think you'll just become a stand-up comedian that will get a standing ovation just because you said \"cum\" in the stage? HELL NO YOU FUCKIN IDIOT, so please shut the fuck up and use words properly you dumb bitch.";

pub async fn copypastas(ctx: &Context, msg: &Message) {
    if msg.channel_id.0 == get_env!("ABB_SHITPOST_CHANNEL", u64) {
        return;
    }

    pasta(ctx, msg, OK_PASTA, "Ok").await;

    if d20::roll_dice("1d20").unwrap().total > 10 {
        return;
    }

    pasta(ctx, msg, "ok", OK_PASTA).await;
    pasta(ctx, msg, "bye", BYE_PASTA).await;
    pasta(ctx, msg, ":moyai:", STONE_PASTA).await;
    pasta(ctx, msg, "based", BASED_PASTA).await;
}

pub async fn pasta(ctx: &Context, msg: &Message, trigger: &str, pasta: &str) {
    if msg.content.to_ascii_lowercase() == trigger.to_ascii_lowercase()
        && msg.author.id.0 != get_env!("ABB_BOT_USER_ID", u64)
    {
        msg.channel_id
            .say(&ctx.http, pasta)
            .await
            .expect("Error sending message");
    }
}
