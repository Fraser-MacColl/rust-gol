use std::collections::HashMap;

/// Position on the grid.
#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }
}

/// Enum for the different states a cell can be in
#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

/// Main game of life simulation struct
pub struct GameOfLife {
    chunks: HashMap<Position, Chunk>,
}

impl GameOfLife {
    pub fn new() -> Self {
        GameOfLife {
            chunks: HashMap::new(),
        }
    }

    // Debug display of chunk info
    pub fn debug_print(&self) {
        println!("Loaded chunks: {}", self.chunks.len());

        for (pos, state) in &self.chunks {
            println!("========");
            println!("X: {}  Y: {}", pos.x, pos.y);
            for y in 0..8 {
                for x in 0..8 {
                    match state.get_cell(x, y) {
                        Ok(Cell::Alive) => print!("#"),
                        Ok(Cell::Dead) => print!("."),
                        Err(_) => print!("E"),
                    }
                }
            }
        }
    }
}

/// Chunk of the GOL simulation area
struct Chunk {
    // Need to keep the old state so it can still be used in neighbour counts of other chunks.
    current: Vec<Vec<Cell>>,
    next: Vec<Vec<Cell>>,
}

impl Chunk {
    /// Initialise a new chunk with the given size.
    pub fn new(size: usize) -> Self {
        Chunk {
            current: vec![vec![Cell::Dead; size]; size],
            next: vec![vec![Cell::Dead; size]; size],
        }
    }

    /// Get the state of a cell in the current state.
    /// Expects the coordinates of the cell within this 8x8 chunk,
    /// returns [`None`] if out of bounds.
    pub fn get_cell(&self, x: usize, y: usize) -> Result<Cell, ()> {
        self.current.get(x).ok_or(())?.get(y).copied().ok_or(())
    }

    /// Set the state of a cell in the next state.
    /// Expects the coordinate of the cell within this 8x8 chunk,
    /// returns [`Err`] if out of bounds.
    pub fn set_cell(&mut self, x: usize, y: usize, state: Cell) -> Result<(), ()> {
        *self.next.get_mut(x).ok_or(())?.get_mut(y).ok_or(())? = state;
        Ok(())
    }
}
