use super::{
    grid::{Cell, Grid},
    model::{GameState, Meta, Move, Pos},
    snake::Snake,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::Level;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub meta: Meta,
    // in case we ever want to state looking back at game states
    // update does not yet push to this because it's not needed
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

    #[tracing::instrument(skip(self, state))]
    pub fn update(&mut self, player_number: usize, modifier: &str, state: GameState) -> Move {
        let mut grid = Grid::new(self.meta.grid_size);
        let (me, others) = tracing::span!(Level::DEBUG, "grid_init").in_scope(|| {
            state
                .food_positions
                .iter()
                .for_each(|f| grid.set(f, Cell::Food));
            let me = Snake::new(player_number, &state.positions[player_number]);
            me.apply(&mut grid, false);

            let others: Vec<_> = state
                .positions
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != player_number)
                .map(|(i, v)| Snake::new(i, v))
                .collect();

            others.iter().for_each(|s| s.apply(&mut grid, false));

            (me, others)
        });

        let my_best_path = if modifier.contains("eager") {
            self.eager_closest_food_in_reach(&state, &grid, &me, &others)
        } else {
            self.full_closest_food_in_reach(&state, &grid, &me, &others)
        };

        match my_best_path {
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

                // pick a random spot next
                grid.successors(&me.head())
                    .first()
                    .map(|p| me.head().direction(p))
                    .unwrap_or(self.bot.last_move)
            }
        }
    }

    #[tracing::instrument(fields(result), skip(self, state, grid))]
    fn full_closest_food_in_reach(
        &self,
        state: &GameState,
        grid: &Grid,
        me: &Snake,
        others: &[Snake],
    ) -> Option<(Vec<Pos>, u32)> {
        let mut my_food_paths: Vec<_> = tracing::span!(Level::DEBUG, "my_search").in_scope(|| {
            state
                .food_positions
                .par_iter()
                .filter_map(|p| me.search(grid, *p).map(|v| (p, v)))
                .collect()
        });

        my_food_paths.sort_by(|a, b| a.1 .1.cmp(&b.1 .1));

        let mut last_option = None;
        for (food, (path, cost)) in my_food_paths {
            last_option = Some((path, cost));
            let other_min_path = tracing::span!(Level::DEBUG, "others_search").in_scope(|| {
                others
                    .par_iter()
                    .filter_map(|s| s.search(grid, *food))
                    .min_by(|a, b| a.1.cmp(&b.1))
            });

            if let Some((_, c)) = other_min_path {
                if cost < c {
                    tracing::debug!("we have a shorter path to {:?} than any other", &food);
                    return last_option;
                }
            } else {
                tracing::debug!("other has no path to food at {:?}", &food);
                return last_option;
            }
        }

        tracing::debug!("not closet to any food or no available path");
        last_option
    }

    #[tracing::instrument(skip(self, state, grid))]
    fn eager_closest_food_in_reach(
        &self,
        state: &GameState,
        grid: &Grid,
        me: &Snake,
        others: &[Snake],
    ) -> Option<(Vec<Pos>, u32)> {
        let mut foods_ordered_by_dist = state.food_positions.clone();
        foods_ordered_by_dist.sort_by(|a, b| a.distance(&me.head()).cmp(&b.distance(&me.head())));

        let mut last_option = None;
        for food in foods_ordered_by_dist {
            if let Some((path, cost)) = me.search(grid, food) {
                last_option = Some((path, cost));

                let other_min_path = others
                    .par_iter()
                    .filter_map(|s| s.search(grid, food))
                    .min_by(|a, b| a.1.cmp(&b.1));

                if let Some((_, c)) = other_min_path {
                    if cost < c {
                        tracing::debug!("we have a shorter path to {:?} than any other", &food);
                        return last_option;
                    }
                } else {
                    tracing::debug!("other has no path to food at {:?}", &food);
                    return last_option;
                }
            }
        }
        tracing::debug!("not closet to any food or no available path");
        last_option
    }
}
