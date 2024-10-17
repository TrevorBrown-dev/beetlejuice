use std::collections::HashMap;
use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ProfanityCheck {
    is_profanity: bool,
    #[allow(dead_code)]
    score: f64,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: serenity::prelude::Context, msg: Message) {
        let client = reqwest::Client::new();
        let mut json = HashMap::new();
        json.insert("message", msg.content.clone());
        let response = client
            .post("https://vector.profanity.dev")
            .json(&json)
            .send()
            .await
            .map_err(|_| "Could not send post request to profanity filter.");

        if let Err(why) = &response {
            println!("{}", why);
            return;
        }

        let response = response.unwrap();

        let response = match response.json::<ProfanityCheck>().await {
            Ok(check) => check,
            Err(why) => {
                println!("{}", why);
                return;
            }
        };

        if response.is_profanity {
            let delete_result = msg.delete(ctx.clone()).await;
            if delete_result.is_err() {
                println!("Could not delete message. \"{}\"", msg.content.clone());
                return;
            }
            let text = format!("That word is a no go!! {}", msg.author.mention());
            if (msg.channel_id.say(&ctx.http, text).await).is_err() {
                println!("Could not reply to message. \"{}\"", msg.content.clone());
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
