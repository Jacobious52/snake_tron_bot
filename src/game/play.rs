use super::model::{GameState, Meta, Move};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub meta: Meta,
    pub game_states: Vec<GameState>,
    pub bot: Bot,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Bot {}

impl Game {
    pub fn new(game_state: GameState) -> Self {
        Self {
            id: game_state.meta.game_id.to_string(),
            meta: game_state.meta.clone(),
            game_states: vec![game_state],
            bot: Bot::default(),
        }
    }

    pub fn init(&mut self) -> Move {
        Move::N
    }

    pub fn update(&mut self, state: GameState) -> Move {
        let food = match state.food.first() {
            Some(f) => f,
            None => return Move::N,
        };

        let pos = state.positions[self.meta.player_number.unwrap()]
            .first()
            .unwrap();

        pos.relative(food)
    }
}
