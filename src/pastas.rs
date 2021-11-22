use serenity::{client::Context, model::channel::Message};

const OK_PASTA: &str = "\"Ok\"? Are you fucking kidding me? I spent a decent portion of my life writing all of that and your response to me is \"Ok\"? Are you so mentally handicapped that the only word you can comprehend is \"Ok\" - or are you just some fucking asshole who thinks that with such a short response, he can make a statement about how meaningless what was written was? Well, I'll have you know that what I wrote was NOT meaningless, in fact, I even had my written work proof-read by several professors of literature. Don't believe me? I doubt you would, and your response to this will probably be \"Ok\" once again. Do I give a fuck? No, does it look like I give even the slightest fuck about two fucking letters? I bet you took the time to type those two letters too, I bet you sat there and chuckled to yourself for 20 hearty seconds before pressing \"send\". You're so fucking pathetic. I'm honestly considering directing you to a psychiatrist, but I'm simply far too nice to do something like that. You, however, will go out of your way to make a fool out of someone by responding to a well-thought-out, intelligent, or humorous statement that probably took longer to write than you can last in bed with a chimpanzee. What do I have to say to you? Absolutely nothing. I couldn't be bothered to respond to such a worthless attempt at a response. Do you want \"Ok\" on your gravestone?";
const BYE_PASTA: &str = "Bye. I'm leaving the server because I'm bored. Despite talking to all of you for a while I don't care about any of you at all. I lied. I'm not 14 I'm 13. And a girl. I can probably draw better than yall as well. Just sayin. Cya.";
const SHIRTLESS_PASTA: &str = "Me, Me when your girl, You, You when your girl, When your girl see, When your girl sees me, When, When Like, When I'm like shirtless, So, So, Soaking wet, Get";
const BASED_PASTA: &str = "Based? Based on what? In your dick? Please shut the fuck up and use words properly you fuckin troglodyte, do you think God gave us a freedom of speech just to spew random words that have no meaning that doesn't even correllate to the topic of the conversation? Like please you always complain about why no one talks to you or no one expresses their opinions on you because you're always spewing random shit like poggers based cringe and when you try to explain what it is and you just say that it's funny like what? What the fuck is funny about that do you think you'll just become a stand-up comedian that will get a standing ovation just because you said \"cum\" in the stage? HELL NO YOU FUCKIN IDIOT, so please shut the fuck up and use words properly you dumb bitch.";
const NFT_PASTA: &str = "You think it’s funny to take screenshots of people’s NFTs, huh? You must be a very immature person to steal someone’s property that they PAID for. Yeah, I said it. You’re the kind of person who thinks that property theft (a seriously illegal offence) is a joke. I don’t even know why you took that screenshot, because you didn’t pay 1000 dollars for it. I did. The blockchain doesn’t lie. Even if you try to save it, it’s my property. You’re just angry that you couldn’t afford this priceless masterpiece. Even if you could, your fingers couldn’t even click fast enough to get one of the 10000 NFTs sold. You’re just mad you don’t own what I own. So, delete that screenshot, or I swear, you’ll be hearing from my lawyers. /s";

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
    pasta(ctx, msg, "me", SHIRTLESS_PASTA).await;
    pasta(ctx, msg, "based", BASED_PASTA).await;
    pasta(ctx, msg, "nft", NFT_PASTA).await;
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
