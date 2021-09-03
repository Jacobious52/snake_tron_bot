use crate::game::{model::GameState, Game};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

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

    pub fn end_game(&mut self, id: &str) {
        self.games.remove(id);
    }
}
