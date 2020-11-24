use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use serenity::framework::{
    StandardFramework,
    standard::macros::{group, command},
    standard::CommandResult,
};

const FEDORA_EMOJI: &str = "<:FedoraHat:780055050697179136>";
const LARM_EMOJI: &str = "<:LArm:780055050725752832>";
const RARM_EMOJI: &str = "<:RArm:780055050332274690>";
const FACE_EMOJI: &str = "<:Face:780055050923278346>";
const LEGS_EMOJI: &str = "<:Legs:780055050663362570>";

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let help_message = format!("
    > Welcome to Homunculus Utopia.\n\
    > \n\
    > You are a little, stupid homunculi - about 6 inches tall.\n\
    > \n\
    > You like to hang out, grow plants, eat food, fight, do dumb things and have no meaning in life.\n\
    > Due to previously stated stupidity, you do not care.\n\
    > \n\
    > Grow some beans.\n\
    > \n\
    > -- The Boys :tm:\n\
    > {pad}{hat}{pad}\n\
    > {larm}{face}{rarm}\n\
    > {pad}{legs}{pad}
    ",
    pad=":black_small_square:", hat=FEDORA_EMOJI, larm=LARM_EMOJI, rarm=RARM_EMOJI, face=FACE_EMOJI, legs=LEGS_EMOJI);
    msg.channel_id.say(&ctx.http, help_message).await?;
    Ok(())
}

#[command]
async fn ugg(ctx: &Context, msg: &Message) -> CommandResult {
    let ugg_message = format!("\"Ugg\" {emoji}", emoji=":gorilla:");
    msg.channel_id.say(&ctx.http, ugg_message).await?;
    Ok(())
}

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
