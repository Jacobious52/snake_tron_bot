use crate::game::{
    model::{GameState, Meta, Move},
    Game,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input {
    Init(Meta),
    Update(GameState),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Output {
    Move(Move),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Games {
    games: HashMap<String, Game>,
}

impl Games {
    pub fn new() -> Self {
        Games {
            games: HashMap::new(),
        }
    }

    pub fn game(&mut self, id: &str) -> Option<&mut Game> {
        self.games.get_mut(id)
    }

    pub fn new_game(&mut self, id: &str, game_state: GameState) -> &mut Game {
        let game = Game::new(game_state);
        self.games.insert(id.to_string(), game);
        self.game(id).unwrap()
    }
}
