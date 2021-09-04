use super::{
    grid::{Cell, Grid},
    model::{GameState, Meta, Move},
    snake::Snake,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub meta: Meta,
    pub game_states: Vec<GameState>,
    #[serde(skip)]
    pub bot: Bot,
}

#[derive(Debug, Clone, Default)]
pub struct Bot {
    last_move: Move,
}

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

    pub fn update(&mut self, player_number: usize, state: GameState) -> Move {
        let mut grid = Grid::new(self.meta.grid_size);

        state.food.iter().for_each(|f| grid.set(f, Cell::Food));
        let me = Snake::new(player_number, &state.positions[player_number]);
        me.apply(&mut grid, false);

        let others: Vec<_> = state
            .positions
            .into_iter()
            .enumerate()
            .filter(|(i, _)| *i != player_number)
            .map(|(i, v)| Snake::new(i, &v))
            .collect();

        others.iter().for_each(|s| s.apply(&mut grid, true));

        // if player_number == 0 {
        //     println!("----\n{}", grid.draw());
        // }

        let closest_food = state
            .food
            .into_iter()
            .min_by(|a, b| me.head().distance(a).cmp(&me.head().distance(b)))
            .unwrap();

        let search = me.search(&grid, closest_food);

        match search {
            Some((path, cost)) => {
                tracing::debug!("found path: cost {:?}", cost);
                let next_move = path[0].direction(&path[1]);
                self.bot.last_move = next_move;
                next_move
            }
            None => {
                tracing::debug!(
                    "no path found, returning last move: {:?}",
                    self.bot.last_move
                );
                self.bot.last_move
            }
        }
    }
}
