use log::*;
use std::env;
use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

fn initialize_log() {
    use env_logger::Env;
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    info!("Logging initialized.");
}

group!({
    name: "general",
    options: {},
    commands: [ping],
});

struct Handler;

impl EventHandler for Handler {}

fn main() {
    initialize_log();

    // Login with a bot token from the environment
    let discord_token = env::var("DISCORD_TOKEN")
        .expect("You must set the DISCORD_TOKEN env variable.");
    let mut client = Client::new(&discord_token, Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}
