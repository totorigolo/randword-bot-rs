use log::*;
use serenity::model::channel::{Reaction, ReactionType};
use serenity::model::id::{ChannelId, MessageId, UserId};
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

pub struct GameStatesManager {
    states: HashMap<ChannelId, GameState>,
    react_to_join_ids: HashMap<MessageId, ChannelId>,
}

pub enum GameState {
    None,
    WaitingForPlayers {
        players: HashSet<UserId>,
    },
    WaitingForVotes {
        players: HashSet<UserId>,
        word: String,
    },
}

impl GameStatesManager {
    pub fn new() -> Self {
        GameStatesManager {
            states: Default::default(),
            react_to_join_ids: Default::default(),
        }
    }

    pub fn status_in(&self, ctx: &Context, channel_id: ChannelId) -> impl Display {
        let state = self.states.get(&channel_id).unwrap_or(&GameState::None);
        match state {
            GameState::None => MessageBuilder::new()
                .push("No game is currently ongoing. ")
                .push("You can start one with ")
                .push_mono("!game start")
                .push(".")
                .build(),
            GameState::WaitingForPlayers { players, .. } => {
                let playing = if !players.is_empty() {
                    let names = players
                        .iter()
                        .map(|id| id.to_user(ctx))
                        .map(|result| {
                            result
                                .map(|user| user.name)
                                .unwrap_or("error fetching name".to_string())
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("Current players: {}.", names)
                } else {
                    "Nobody joined yet.".into()
                };

                MessageBuilder::new()
                    .push("A game has started, and I am waiting for ")
                    .push("players to join. ")
                    .push("If you want to play, react with ")
                    .push(":+1:")
                    .push(" on my previous message. ")
                    .push(playing)
                    .build()
            }
            GameState::WaitingForVotes { .. } => MessageBuilder::new()
                .push("A game is ongoing, and I am waiting for ")
                .push("your votes by direct messages. ")
                .push("To vote, follow the instructions ")
                .push("I sent you by DM.")
                .build(),
        }
    }

    pub fn start_in(&mut self, channel_id: ChannelId) -> Result<(), impl Display> {
        let prev_state = self.states.remove(&channel_id).unwrap_or(GameState::None);
        match prev_state {
            GameState::None => {
                let state = GameState::WaitingForPlayers {
                    players: Default::default(),
                };
                self.states.insert(channel_id, state);
                Ok(())
            }
            _ => Err(MessageBuilder::new()
                .push("A game is already ongoing, you must ")
                .push_mono("!game stop")
                .push(" it first.")
                .build()),
        }
    }

    pub fn reaction_add(&mut self, reaction: Reaction) {
        trace!("Processing reaction: {:?}", reaction);

        // TODO: Filter before calling this fn
        match &reaction.emoji {
            ReactionType::Unicode(s) if s == ":+1:" => {}
            _ => return,
        }

        let mut remove_sub = true;

        if let Some(channel_id) = self.react_to_join_ids.get(&reaction.message_id) {
            self.states.get_mut(&channel_id).map(|state| match state {
                GameState::WaitingForPlayers { players, .. } => {
                    debug!("Adding {} to game in {}.", reaction.user_id, channel_id);

                    players.insert(reaction.user_id);
                    remove_sub = false;
                }
                _ => {}
            });
        }

        if remove_sub {
            self.unsubscribe(&reaction.message_id);
        }
    }

    pub fn subscribe(&mut self, message_id: MessageId, channel_id: ChannelId) {
        trace!(
            "Subscribed to reactions to {} for channel {}.",
            message_id,
            channel_id
        );
        self.react_to_join_ids.insert(message_id, channel_id);
    }

    pub fn unsubscribe(&mut self, message_id: &MessageId) {
        self.react_to_join_ids.remove(&message_id);
    }
}
