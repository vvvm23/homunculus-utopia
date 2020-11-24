use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const FEDORA_EMOJI: &str = "<:FedoraHat:780055050697179136>";
const LARM_EMOJI: &str = "<:LArm:780055050725752832>";
const RARM_EMOJI: &str = "<:RArm:780055050332274690>";
const FACE_EMOJI: &str = "<:Face:780055050923278346>";
const LEGS_EMOJI: &str = "<:Legs:780055050663362570>";

const HELP_COMMAND: &str = "!help";
const UGG_COMMAND: &str = "!ugg";

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("-> {}", msg.content);
        if msg.content == HELP_COMMAND {
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

            if let Err(e) = msg.channel_id.say(&ctx.http, help_message).await {
                println!("! Error sending message: {:?}", e);
            }
        } else if msg.content == UGG_COMMAND {
            let ugg_message = format!("\"Ugg\" {emoji}", emoji=":gorilla:");
            if let Err(e) = msg.channel_id.say(&ctx.http, ugg_message).await {
                println!("! Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("> {} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("! No token found in environmental variables.\nTry `export DISCORD_TOKEN=abcd123`");
    
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("! Error encountered creating client");

    if let Err(e) = client.start().await {
        println!("! Client threw error: {:?}", e);
    }
}
