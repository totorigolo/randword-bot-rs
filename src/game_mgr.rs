use log::*;
use serenity::model::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct GameStatesManager {
    states: HashMap<ChannelId, GameState>,
    react_subscriptions: HashMap<MessageId, ChannelId>,
}

#[derive(Debug)]
enum GameState {
    None,
    WaitingForPlayers {
        players: HashSet<UserId>,
        react_subscribed_to: Vec<MessageId>,
    },
    WaitingForVotes {
        players: HashSet<UserId>,
        word: String,
    },
}

/// Represents the state of a game. This enum is returned by the function
/// `GameStatesManager::status_in`.
#[derive(Debug)]
pub enum GameStatus<'a> {
    NoGame,
    WaitingForPlayers { players: &'a HashSet<UserId> },
    WaitingForVotes { players: &'a HashSet<UserId> },
}

#[derive(Debug)]
pub enum GameStartResult {
    Started,
    AlreadyOngoing,
}

impl Default for GameStatesManager {
    fn default() -> Self {
        GameStatesManager {
            states: Default::default(),
            react_subscriptions: Default::default(),
        }
    }
}

impl GameStatesManager {
    pub fn new() -> Self {
        GameStatesManager::default()
    }

    pub fn status_in(&self, channel_id: ChannelId) -> GameStatus {
        let state = self.states.get(&channel_id).unwrap_or(&GameState::None);
        match state {
            GameState::None => GameStatus::NoGame,
            GameState::WaitingForPlayers { players, .. } => {
                GameStatus::WaitingForPlayers { players: &players }
            }
            GameState::WaitingForVotes { players, .. } => {
                GameStatus::WaitingForPlayers { players: &players }
            }
        }
    }

    pub fn start_in(&mut self, channel_id: ChannelId) -> GameStartResult {
        let prev_state = self.states.remove(&channel_id).unwrap_or(GameState::None);
        match prev_state {
            GameState::None => {
                let state = GameState::WaitingForPlayers {
                    players: Default::default(),
                    react_subscribed_to: Default::default(),
                };
                self.states.insert(channel_id, state);

                GameStartResult::Started
            }
            _ => GameStartResult::AlreadyOngoing,
        }
    }

    pub fn reaction_add(&mut self, reaction: Reaction) {
        trace!("Processing reaction add: {:?}", reaction);

        // TODO: Filter before calling this fn
        match &reaction.emoji {
            ReactionType::Unicode(s) if s == ":+1:" => {}
            _ => return,
        }

        let mut remove_sub = true;

        if let Some(channel_id) = self.react_subscriptions.get(&reaction.message_id) {
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
            self.unsubscribe(reaction.message_id);
        }
    }

    pub fn reaction_remove(&mut self, reaction: Reaction) {
        trace!("Processing reaction add: {:?}", reaction);
        unimplemented!("reaction_remove")
    }

    pub fn subscribe(&mut self, message_id: MessageId, channel_id: ChannelId) {
        trace!(
            "Subscribed to reactions on {} in channel {}.",
            message_id,
            channel_id
        );
        self.react_subscriptions.insert(message_id, channel_id);
    }

    pub fn unsubscribe(&mut self, message_id: MessageId) {
        self.react_subscriptions.remove(&message_id);
    }
}
