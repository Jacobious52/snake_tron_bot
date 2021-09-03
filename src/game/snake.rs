use pathfinding::prelude::astar;

use super::{
    grid::{Cell, Grid},
    model::Pos,
};

pub struct Snake {
    positions: Vec<Pos>,
}

impl Snake {
    pub fn new(positions: &[Pos]) -> Self {
        Snake {
            positions: positions.to_vec(),
        }
    }

    pub fn head(&self) -> Pos {
        self.positions[0]
    }

    pub fn body(&self) -> &[Pos] {
        &self.positions[1..]
    }

    pub fn apply(&self, grid: &mut Grid) {
        grid.set(&self.head(), Cell::Head);
        self.body().iter().for_each(|p| grid.set(p, Cell::Body));
    }

    pub fn search(&self, grid: &Grid, target: Pos) -> Option<(Vec<Pos>, u32)> {
        astar(
            &self.head(),
            |p| {
                grid.neighbours(p)
                    .into_iter()
                    .filter(|p| grid.unblocked(p))
                    .map(|p| (p, 1))
                    .collect::<Vec<_>>()
            },
            |p| p.distance(&target) / 3,
            |p| *p == target,
        )
    }
}
