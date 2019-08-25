use std::fmt::Display;
use std::collections::HashMap;
use serenity::model::id::{ChannelId, UserId};
use serenity::utils::MessageBuilder;

pub struct GameStatesManager {
    pub states: HashMap<ChannelId, GameState>,
}

pub enum GameState {
    None,
    WaitingForPlayers {
        players: Vec<UserId>,
    },
    WaitingForVotes {
        players: Vec<UserId>,
        word: String,
    },
}

impl GameStatesManager {
    pub fn new() -> Self {
        GameStatesManager {
            states: Default::default(),
        }
    }

    pub fn start_on(&mut self, channel_id: ChannelId) -> Result<(), impl Display> {
        let prev_state = self
            .states
            .remove(&channel_id)
            .unwrap_or(GameState::None);
        match prev_state {
            GameState::None => {
                let state = GameState::WaitingForPlayers {
                    players: vec![],
                };
                self.states.insert(channel_id, state);
                Ok(())
            }
            _ => {
                Err(MessageBuilder::new()
                    .push("A game is already ongoing, you must ")
                    .push_mono("!game stop")
                    .push(" it first.")
                    .build())
            }
        }
    }
}

