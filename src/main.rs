use serenity::all::MessageUpdateEvent;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;

const PROFANITY_LIST_ADDITIONS: [&str; 10] = [
    "rizz", "skibidi", "ohio", "gyatt", "sigma", "fanum", "bruh", "yeet", "simp", "pomni",
];

struct Handler;

async fn check_profanity(msg: String) -> Result<bool, String> {
    let text = format!(
        "https://www.purgomalum.com/service/containsprofanity?text={}&add={}",
        msg,
        PROFANITY_LIST_ADDITIONS.join(",")
    );
    let response = reqwest::get(text)
        .await
        .map_err(|_| "Profanity check api returned bad result.")?
        .text()
        .await
        .map_err(|_| "Could not get text from profanity api call.")?;

    response
        .parse::<bool>()
        .map_err(|_| "Could not parse profanity api call response to bool.".to_string())
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: serenity::prelude::Context, msg: Message) {
        let is_profane = match check_profanity(msg.content.clone()).await {
            Ok(is_profane) => is_profane,
            Err(why) => {
                println!("NEW: {}", why);
                return;
            }
        };

        if is_profane {
            let delete_result = msg.delete(ctx.clone()).await;
            if delete_result.is_err() {
                println!("NEW: Could not delete message. \"{}\"", msg.content.clone());
                return;
            }
            let text = format!("That word is a no go!! {}", msg.author.mention());
            if (msg.channel_id.say(&ctx.http, text).await).is_err() {
                println!(
                    "NEW: Could not reply to message. \"{}\"",
                    msg.content.clone()
                );
                return;
            }
        }
    }

    async fn message_update(
        &self,
        ctx: serenity::prelude::Context,
        _old_if_available: Option<Message>,
        _new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        let mut msg = Message::default();
        event.apply_to_message(&mut msg);

        let is_profane = match check_profanity(msg.content.clone()).await {
            Ok(is_profane) => is_profane,
            Err(why) => {
                println!("EDIT: {}", why);
                return;
            }
        };

        if is_profane {
            let delete_result = msg.delete(ctx.clone()).await;
            if delete_result.is_err() {
                println!(
                    "EDIT: Could not delete message. \"{}\"",
                    msg.content.clone()
                );
                return;
            }
            let text = format!("That word is a no go!! {}", msg.author.mention());
            if (msg.channel_id.say(&ctx.http, text).await).is_err() {
                println!(
                    "EDIT: Could not reply to message. \"{}\"",
                    msg.content.clone()
                );
                return;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
