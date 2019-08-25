use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use crate::GameStatesContainer;
use serenity::utils::MessageBuilder;

#[command]
#[description = "Starts a game if none is ongoing."]
pub fn start(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let mut reply = MessageBuilder::new();

    let mut data = ctx.data.write();
    if let Some(game_mgr) = data.get_mut::<GameStatesContainer>() {
        match game_mgr.start_on(msg.channel_id) {
            Ok(()) => {
                reply
                    .push("The game is starting. React with ")
                    .push(":+1:")
                    .push(" on this message if you want to take part.")
                    .push("You can remove the emoji if you change your mind.");
            }
            Err(err) => {
                reply.push("Error: ").push(err);
                let _ = msg.react(&ctx, ":x:");
            }
        }
    } else {
        reply.push("Error: Failed to fetch game data.");
        let _ = msg.react(&ctx, ":x:");
    }

    let _ = msg.channel_id.say(&ctx.http, reply);
    Ok(())
}
