use pathfinding::prelude::absdiff;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Pos(pub i64, pub i64);

impl Pos {
    pub fn distance(&self, other: &Pos) -> u32 {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
    }

    pub fn direction(&self, other: &Pos) -> Move {
        match self.0.cmp(&other.0) {
            std::cmp::Ordering::Less => Move::E,
            std::cmp::Ordering::Equal => match self.1.cmp(&other.1) {
                std::cmp::Ordering::Less => Move::S,
                std::cmp::Ordering::Equal => todo!(),
                std::cmp::Ordering::Greater => Move::N,
            },
            std::cmp::Ordering::Greater => Move::W,
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

impl From<(i64, i64)> for Pos {
    fn from(t: (i64, i64)) -> Self {
        Pos(t.0, t.1)
    }
}

impl From<&(i64, i64)> for Pos {
    fn from(t: &(i64, i64)) -> Self {
        Pos(t.0, t.1)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Move {
    N,
    S,
    E,
    W,
}

impl Default for Move {
    fn default() -> Self {
        Self::W
    }
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
