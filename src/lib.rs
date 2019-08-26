#[macro_use]
mod macros;
mod commands;
mod game_mgr;
mod utils;

use log::*;
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    model::{channel::Reaction, event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};

use commands::{game::*, meta::*, owner::*};
use game_mgr::*;

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
        info!("Connected as '{}'", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let mut data = ctx.data.write();
        let game_mgr = get_mut!(data, GameStatesContainer);

        game_mgr.reaction_add(reaction);
    }

    fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        let mut data = ctx.data.write();
        let game_mgr = get_mut!(data, GameStatesContainer);

        game_mgr.reaction_remove(reaction);
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
    commands: [status, start]
});

pub fn run() {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN env variable not set.");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<GameStatesContainer>(GameStatesManager::new());
    }

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);
            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix("!"))
            .group(&GENERAL_GROUP)
            .group(&GAME_GROUP)
            .help(&HELP),
    );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
