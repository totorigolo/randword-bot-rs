use std::env;
use log::*;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
	Result,
};

struct Handler;

impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message) {
		if let Err(why) = self.message_impl(ctx, msg) {
			error!("Error sending message: {:?}", why);
		}
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

impl Handler {
	fn message_impl(&self, ctx: Context, msg: Message) -> Result<()> {
		let trimmed = msg.content.trim();
        if trimmed == "!ping" {
			msg.channel_id.say(&ctx.http, "Pong!")?;
		}

		Ok(())
	}
}

fn initialize_log() {
    use env_logger::Env;
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    info!("Logging initialized.");
}

fn main() {
	initialize_log();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}

