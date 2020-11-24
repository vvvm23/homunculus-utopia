use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use serenity::framework::{
    StandardFramework,
    standard::macros::group,
};

mod commands;
use commands::*;

#[group]
#[commands(ping, help, ugg)]
struct General;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _: Context, msg: Message) {
        println!("-> {}\n", msg.content);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("> {} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("! No token found in environmental variables.\nTry `export DISCORD_TOKEN=abcd123`");
    
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("! Error encountered creating client");

    if let Err(e) = client.start().await {
        println!("! Client threw error: {:?}", e);
    }
}
