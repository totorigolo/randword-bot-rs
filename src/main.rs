//! Requires the 'framework' feature flag be enabled in your project's
//! `Cargo.toml`.
//!
//! This can be enabled by specifying the feature in the dependency section:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["framework", "standard_framework"]
//! ```
mod commands;
mod game_state;

use std::{
    collections::HashSet,
    env,
    sync::Arc,
};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use log::*;

use commands::{
    meta::*,
    owner::*,
    game::*,
};
use game_state::*;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct GameStatesContainer;

impl TypeMapKey for GameStatesContainer {
	type Value = GameStatesManager;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

group!({
    name: "general",
    options: {},
    commands: [ping, quit]
});

group!({
    name: "game",
    options: {
		prefix: "game",
        description: "Game related commands.",
	},
    commands: [start]
});

fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    kankyo::load().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    env_logger::init();

    let token = env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN env variable not set.");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();

        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));

		let mgr = GameStatesManager::new();
		//data.insert::<GameStatesContainer>(Arc::new(Mutex::new(mgr)));
		data.insert::<GameStatesContainer>(mgr);
    }

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);
            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .prefix("!"))
        .group(&GENERAL_GROUP)
        .group(&GAME_GROUP)
		.help(&HELP));

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
