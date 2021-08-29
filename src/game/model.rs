use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Pos(i64, i64);

impl Pos {
    pub fn abs(&self) -> Pos {
        Pos(self.0.abs(), self.1.abs())
    }

    pub fn relative(&self, other: &Pos) -> Move {
        if self.0 < other.0 {
            return Move::W;
        } else if self.0 > other.0 {
            return Move::E;
        }

        if self.1 < other.1 {
            return Move::W;
        } else if self.1 > other.1 {
            return Move::E;
        }

        Move::N
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
