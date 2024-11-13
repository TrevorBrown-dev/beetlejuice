use serenity::all::{EditChannel, GuildChannel, MessageUpdateEvent};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;

const PROFANITY_LIST_ADDITIONS: [&str; 9] = [
    "rizz", "skibidi", "gyatt", "sigma", "fanum", "bruh", "yeet", "simp", "pomni",
];
struct Handler {
    words_said: Vec<String>,
    
}

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
const ENABLE_FUCK_CHARLIE: bool = false;
const CHARLIE: &str = "132581846566436864";

const ENABLE_WORD_GAME: bool = true;
const ENABLE_PROFANITY_CHECK: bool = false;
#[async_trait]
impl EventHandler for Handler {
    
    async fn message(&self, ctx: serenity::prelude::Context, msg: Message) {
        if (ENABLE_PROFANITY_CHECK){

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

            if(ENABLE_FUCK_CHARLIE) {
                //check if charlie sent the message
                if msg.author.id.to_string() == CHARLIE {
                    let delete_result = msg.delete(ctx.clone()).await;
                    if delete_result.is_err() {
                        println!("NEW: Could not delete message from Charlie. \"{}\"", msg.content.clone());
                        return;
                    }
                    let text = format!("That word is a no go!! {}", msg.author.mention());
                    if (msg.channel_id.say(&ctx.http, text).await).is_err() {
                        println!(
                            "NEW: Could not reply to Charlie's message. \"{}\"",
                            msg.content.clone()
                        );
                        return;
                    }
                }
            }
        


        if(ENABLE_WORD_GAME) {
            let has_been_said = false;
            //split the message into words
            let words = msg.content.split_whitespace();
            for word in words {
                //check if the word has been said before
                if self.words_said.contains(&word.to_string()) {
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
                    return;
                }
            }


           
        }
    }
    async fn channel_update(
        &self,
        ctx: serenity::prelude::Context,
        old: Option<GuildChannel>,
        mut new: GuildChannel,
    ) {
        // Check if old channel name was `Ohio`
        // If so, change it to `Ohio`
        // If not, do nothing
        if let Some(old) = old {
            if old.name.to_lowercase() == "ohio" && new.name.to_lowercase() != "ohio" {
                let text = format!("{} changed to {}", new.name, "Ohio");

                let builder = EditChannel::new().name("Ohio");
                if (new.edit(&ctx.http, builder).await).is_err() {
                    println!("CHANNEL UPDATE: Could not edit channel. \"{}\"", new.name);
                    return;
                }

                if (new.id.say(&ctx.http, text).await).is_err() {
                    println!(
                        "CHANNEL UPDATE: Could not reply to message. \"{}\"",
                        new.name
                    );
                    return;
                }
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
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS;

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
