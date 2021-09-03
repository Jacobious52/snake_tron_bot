use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Pos(i64, i64);

impl Pos {
    pub fn relative(&self, other: &Pos) -> Move {
        match self.0.cmp(&other.0) {
            std::cmp::Ordering::Less => Move::E,
            std::cmp::Ordering::Greater => Move::W,
            std::cmp::Ordering::Equal => match self.1.cmp(&other.1) {
                std::cmp::Ordering::Less => Move::S,
                std::cmp::Ordering::Equal => Move::N,
                std::cmp::Ordering::Greater => Move::N,
            },
        }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Move {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub tick: usize,
    pub positions: Vec<Vec<Pos>>,
    pub food: Vec<Pos>,
    pub last_moves: Vec<Move>,
    pub player_alive: Vec<bool>,
    pub meta: Meta,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub player_number: Option<usize>,
    pub grid_size: usize,
    pub player_count: usize,
    pub game_id: String,
}
