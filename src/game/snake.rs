use pathfinding::prelude::astar;

use super::{
    grid::{Cell, Grid},
    model::Pos,
};

#[derive(Debug, Clone)]
pub struct Snake {
    player_number: usize,
    positions: Vec<Pos>,
}

impl Snake {
    pub fn new(player_number: usize, positions: &[Pos]) -> Self {
        Snake {
            player_number,
            positions: positions.to_vec(),
        }
    }

    pub fn head(&self) -> Pos {
        self.positions[0]
    }

    pub fn body(&self) -> &[Pos] {
        &self.positions[1..]
    }

    pub fn player_number(&self) -> usize {
        self.player_number
    }

    pub fn apply(&self, grid: &mut Grid, apply_future: bool) {
        grid.set(&self.head(), Cell::Head);
        self.body().iter().for_each(|p| grid.set(p, Cell::Body));

        if apply_future {
            grid.successors(&self.head())
                .iter()
                .for_each(|p| grid.set(p, Cell::FutureHead));
        }
    }

    pub fn search(&self, grid: &Grid, target: Pos) -> Option<(Vec<Pos>, u32)> {
        astar(
            &self.head(),
            |p| {
                grid.successors(p)
                    .into_iter()
                    .map(|p| (p, 1))
                    .collect::<Vec<_>>()
            },
            |p| p.distance(&target) / 3,
            |p| *p == target,
        )
    }
}
