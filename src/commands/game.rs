use log::*;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use crate::GameStatesContainer;
use serenity::utils::MessageBuilder;

#[command]
#[description = "Prints the status of the game in the channel."]
pub fn status(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let mut reply = MessageBuilder::new();
    let data = ctx.data.read();
    if let Some(game_mgr) = data.get::<GameStatesContainer>() {
        let status = game_mgr.status_in(&ctx, msg.channel_id);
        reply.push(status);
    } else {
        reply.push("Error: Failed to fetch game data.");
        error!("Failed to fetch game_mgr.");
        let _ = msg.react(&ctx, ":skull:");
    }

    let _ = msg.channel_id.say(&ctx.http, reply);
    Ok(())
}

#[command]
#[description = "Starts a game if none is ongoing."]
pub fn start(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let mut data = ctx.data.write();
    if let Some(game_mgr) = data.get_mut::<GameStatesContainer>() {
        match game_mgr.start_in(msg.channel_id) {
            Ok(()) => {
                let reply = MessageBuilder::new()
                    .push("The game is starting. React with ")
                    .push(":+1:")
                    .push(" on this message if you want to take part.")
                    .push("You can remove the emoji if you change your mind.")
                    .build();

                if let Ok(reply_msg) = msg
                    .channel_id
                    .say(&ctx.http, reply) {
                    game_mgr.subscribe(reply_msg.id, msg.channel_id);
                }
            }
            Err(err) => {
                let _ = msg.channel_id.say(&ctx.http, err);
                let _ = msg.react(&ctx, ":x:");
            }
        }
    } else {
        let _ = msg.channel_id.say(&ctx.http,
            "Error: Failed to fetch game data.");
        error!("Failed to fetch game_mgr.");
        let _ = msg.react(&ctx, ":skull:");
    }

    Ok(())
}
