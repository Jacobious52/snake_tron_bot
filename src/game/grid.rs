use super::model::Pos;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Food,
    Head,
    Body,
    FutureHead,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Cell::Empty => write!(f, "."),
            Cell::Food => write!(f, "o"),
            Cell::Head => write!(f, "X"),
            Cell::Body => write!(f, "#"),
            Cell::FutureHead => write!(f, "^"),
        }
    }
}

pub struct Grid {
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        let grid = vec![vec![Cell::Empty; size]; size];
        Grid { grid }
    }

    pub fn get(&self, pos: &Pos) -> Cell {
        self.grid[pos.0 as usize][pos.1 as usize]
    }

    pub fn set(&mut self, pos: &Pos, cell: Cell) {
        self.grid[pos.0 as usize][pos.1 as usize] = cell;
    }

    pub fn draw(&self) -> String {
        let mut output = String::new();
        for col in &self.grid {
            for c in col {
                output += &c.to_string();
            }
            output += "\n";
        }
        output
    }

    pub fn successors(&self, pos: &Pos) -> Vec<Pos> {
        self.neighbours(pos)
            .into_iter()
            .filter(|p| self.unblocked(p))
            .collect()
    }

    pub fn unblocked(&self, pos: &Pos) -> bool {
        let cell = self.get(pos);
        matches!(cell, Cell::Empty | Cell::Food)
    }

    pub fn neighbours(&self, pos: &Pos) -> Vec<Pos> {
        let delta = [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)];

        delta
            .iter()
            .map(|d| *pos + d.into())
            .filter(|p| self.inside(p))
            .collect()
    }

    fn inside(&self, pos: &Pos) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.grid.len() as i64 && pos.1 < self.grid.len() as i64
    }
}
