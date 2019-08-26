use crate::game_mgr::{GameStartResult, GameStatus};
use crate::utils::ToUsername;
use crate::GameStatesContainer;
use log::*;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::fmt;

const OK_EMOJI: &str = ":white_check_mark:";
const FAIL_EMOJI: &str = ":x:";
const DEAD_EMOJI: &str = ":skull:";

#[command]
#[description = "Prints the status of the game in the channel."]
pub fn status(ctx: &mut Context, msg: &Message, _: Args) -> CommandResult {
    let data = ctx.data.read();
    let game_mgr = get!(data, GameStatesContainer);

    let status = game_mgr.status_in(msg.channel_id);
    let reply = status_to_string(&status, &ctx);
    
    let _ = msg.channel_id.say(&ctx.http, reply);
    let _ = msg.react(&ctx, OK_EMOJI);

    Ok(())
}

#[command]
#[description = "Starts a game if none is ongoing."]
pub fn start(ctx: &mut Context, msg: &Message, _: Args) -> CommandResult {
    let mut data = ctx.data.write();
    let game_mgr = get_mut!(data, GameStatesContainer);

    let start_result = game_mgr.start_in(msg.channel_id);
    let reply = start_result_to_string(&start_result);

    match start_result {
        GameStartResult::Started => {
            match msg.channel_id.say(&ctx.http, reply) {
                Ok(reply_msg) => {
                    game_mgr.subscribe(reply_msg.id, msg.channel_id);
                    let _ = msg.react(&ctx, OK_EMOJI);
                }
                Err(err) => {
                    error!("Failed to send reply to game_start: {:?}", err);
                    let _ = msg.react(&ctx, DEAD_EMOJI);
                }
            }
        }
        GameStartResult::AlreadyOngoing => {
            let _ = msg.channel_id.say(&ctx.http, reply);
            let _ = msg.react(&ctx, FAIL_EMOJI);
        }
    }

    Ok(())
}

/*
////////////////////////////////////////////////////////////////////////////////
*/

#[cfg(not(feature = "translated"))]
fn status_to_string(status: &GameStatus, to_username: &impl ToUsername) -> impl fmt::Display {
    use GameStatus::*;
    match status {
        NoGame => MessageBuilder::new()
            .push("No game is currently ongoing. You can start one with ")
            .push_mono("!game start")
            .push(".")
            .build(),
        WaitingForPlayers { players, .. } => MessageBuilder::new()
            .push("A game has started, and I am waiting for players ")
            .push("to join. If you want to play, react with ")
            .push(":+1:")
            .push(" on my previous message. ")
            .push(if !players.is_empty() {
                format!(
                    "Current players: {}.",
                    players
                        .iter()
                        .map(|id| to_username.to_username(*id))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            } else {
                String::from("Nobody joined yet.")
            })
            .build(),
        WaitingForVotes { .. } => MessageBuilder::new()
            .push("A game is ongoing, and I am waiting for your votes by ")
            .push("direct messages. To vote, follow the instructions I ")
            .push("sent you by DM.")
            .build(),
    }
}

#[cfg(feature = "fr")]
fn status_to_string(status: &GameStatus, to_username: &impl ToUsername) -> impl fmt::Display {
    use GameStatus::*;
    match status {
        NoGame => MessageBuilder::new()
            .push("Il n'y a aucun jeu en cours actuellement. Vous pouvez en ")
            .push("démarrer un avec ")
            .push_mono("!game start")
            .push(".")
            .build(),
        WaitingForPlayers { players, .. } => MessageBuilder::new()
            .push("Un jeu a démarré, et j'attend actuellement que les joueurs ")
            .push("s'inscrivent. Si vous voulez jouer, réagissez avec ")
            .push(":+1:")
            .push(" Sur mon précédent message. ")
            .push(if !players.is_empty() {
                format!(
                    "Sont actuellement inscrits : {}.",
                    players
                        .iter()
                        .map(|id| to_username.to_username(*id))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            } else {
                String::from("Personne ne s'est encore inscrit.")
            })
            .build(),
        WaitingForVotes { .. } => MessageBuilder::new()
            .push("Un jeu est en cours, et je suis en train d'attendre vos ")
            .push("votes par message direct. Pour voter, suivez les ")
            .push("instructions que je vous ai envoyées.")
            .build(),
    }
}

#[cfg(not(feature = "translated"))]
fn start_result_to_string(start_result: &GameStartResult) -> impl fmt::Display {
    match start_result {
        GameStartResult::Started => MessageBuilder::new()
            .push("The game is starting. React with ")
            .push(":+1:")
            .push(" on this message if you want to take part.")
            .push("You can remove the emoji if you change your mind.")
            .build(),
        GameStartResult::AlreadyOngoing => MessageBuilder::new()
            .push("A game is already ongoing, you must ")
            .push_mono("!game stop")
            .push(" it first.")
            .build(),
    }
}

#[cfg(feature = "fr")]
fn start_result_to_string(start_result: &GameStartResult) -> impl fmt::Display {
    match start_result {
        GameStartResult::Started => MessageBuilder::new()
            .push("Le jeu démarre. Réagissez avec ")
            .push(":+1:")
            .push(" sur ce message pour participer. Vous pouvez également")
            .push("enlever l'emoji si vous changez d'avis.")
            .build(),
        GameStartResult::AlreadyOngoing => MessageBuilder::new()
            .push("Un jeu est déjà en cours. Vous devez l'arrêter avec ")
            .push_mono("!game stop")
            .push(" avant de pouvoir le démarrer.")
            .build(),
    }
}
