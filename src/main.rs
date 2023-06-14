use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
const PREFIX: &str = ".";

const HELP_MESSAGE: &str = "insert help message";

const HELP_COMMAND: &str = ".help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
            if msg.content.starts_with(PREFIX) == true {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Use .help for available commands.").await {
                    println!("Error sending message: {:?}", why);
            };
           /*  if msg.content == HELP_COMMAND {
             if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
               println!("Error sending message: {:?}", why);
            }  */
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}