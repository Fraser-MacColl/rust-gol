//! Module to hold logic for the Game of Life simulation.

/// Enum to represent each cell in the Game of Life world.
/// Each cell can only either be alive or dead, and this
/// is codified by only having the two enum variants.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum Cell {
    #[default]
    Dead,
    Alive,
}



/// Main Game of Life simulation struct.
pub struct GameOfLife {
    regions: Vec<Region>
}

impl GameOfLife {
    /// Create a new empty world.
    pub fn new() -> GameOfLife {
        !unimplemented!()
    }

    /// Step the simulation to the next state.
    pub fn step(&mut self) {
        !unimplemented!()
        // Step each region
        // Split Regions that have disjoint cells
        // Merge regions that are too close
    }

    /// Get the state of the cell at the given x y coordinates.
    pub fn get_cell(&self, x: isize, y: isize) -> Cell {
        !unimplemented!()
    }

    /// Populate the provided region with the state of the current world.
    pub fn populate_region(&self, region: &mut Region) {
        !unimplemented!()
    }

    /// Set the state of the world to that of the given region.
    pub fn set_region(&mut self, region: &Region) {
        !unimplemented!()
    }
}



/// Structure to hold the state of a 2D region of a Game of Life world.
/// The x y position is the -x -y corner of the region,
/// and the width and height are always positive, growing in the positive x and y direction.
pub struct Region {
    x: isize,
    y: isize,
    width: usize,
    height: usize,
    state: Vec<Vec<Cell>>
}

impl Region {
    /// Create a new all dead region.
    pub fn new(x: isize, y: isize, width: usize, height: usize) -> Region {
        Region {
            x, y, width, height,
            state: vec![vec![Cell::Dead; height]; width]
        }
    }

    /// Check if a position is in the bounds of this region.
    fn pos_in_bounds(&self, x: isize, y: isize) -> bool {
        if x < self.x { return false };
        if y < self.y { return false };
        if let Some(num) = self.x.checked_add_unsigned(self.width) {
            if x >= num { return false }
        } else { return false }
        if let Some(num) = self.y.checked_add_unsigned(self.height) {
            if y >= num { return false }
        } else { return false }

        // All bound checks passed
        true
    }

    /// Turn world coordinates into local coordinates within this region's internal buffer.
    fn pos_to_local(&self, x: isize, y: isize) -> Option<(usize, usize)> {
        if !self.pos_in_bounds(x, y) { return None }
        Some(((x-self.x) as usize, (y-self.y) as usize))
    }

    /// Set the state of a specific cell.
    /// The x y position is in world coordinates, not the local coordinates of the region.
    /// If the x y position is outside this region, this function will fail silently.
    pub fn set_cell(&mut self, x: isize, y: isize, state: Cell) {
        if !self.pos_in_bounds(x, y) { return }
        let Some((x, y)) = self.pos_to_local(x, y)
        else { return };

        self.state[x][y] = state;
    }

    /// Adjust the width or height of the region.
    /// New space is filled with dead cells, while reducing the size truncates the cells.
    /// If adjusting the -x or -y edges, the position will be adjusted accordingly.
    pub fn adjust_edge(&mut self, edge: Edge, amount: isize) {
        !unimplemented!()
    }

    /// Move the region by the given amount in the x and y directions.
    /// New cells will be filled with dead cells, and old cells will be truncated.
    pub fn move_region(&mut self, x: isize, y: isize) {
        !unimplemented!()
    }

    // GETTERS
    // Can't just make members public as there are invariants with the vec to maintain.
    pub fn x(&self) -> isize { self.x }
    pub fn y(&self) -> isize { self.y }
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    pub fn state(&self) -> &Vec<Vec<Cell>> { &self.state }
}

#[cfg(test)]
mod region_tests {
    use super::*;

    #[test]
    fn pos_in_bounds() {
        // Region going from (-5, -5) up to (5, 5) inclusive
        let region = Region::new(-5, -5, 11, 11);

        // In bounds
        assert!(region.pos_in_bounds(0, 0));
        assert!(region.pos_in_bounds(-3, 2));
        assert!(region.pos_in_bounds(-5, 5));
        assert!(region.pos_in_bounds(-5, -5));
        assert!(region.pos_in_bounds(5, 5));
        assert!(region.pos_in_bounds(5, -5));

        // Out of bounds
        assert!(!region.pos_in_bounds(15, 4));
        assert!(!region.pos_in_bounds(0, 8));
        assert!(!region.pos_in_bounds(6, 5));
        assert!(!region.pos_in_bounds(5, -6));
        assert!(!region.pos_in_bounds(-6, 6));
        assert!(!region.pos_in_bounds(-5, -6));
    }

    #[test]
    fn pos_to_local() {
        // Region going from (-5, -5) up to (5, 5) inclusive
        let region = Region::new(-5, -5, 11, 11);

        // Outside region
        assert_eq!(None, region.pos_to_local(6, 5));
        assert_eq!(None, region.pos_to_local(10, -3));
        assert_eq!(None, region.pos_to_local(-2, 6));

        // Inside region
        assert_eq!(Some((0, 0)), region.pos_to_local(-5, -5));
        assert_eq!(Some((10, 10)), region.pos_to_local(5, 5));
        assert_eq!(Some((8, 3)), region.pos_to_local(3, -2));
    }

    #[test]
    fn set_cell() {
        // Region going from (-5, -5) up to (5, 5) inclusive
        let mut region = Region::new(-5, -5, 11, 11);

        // Outside region
        region.set_cell(-6, 3, Cell::Alive);
        region.set_cell(2, 6, Cell::Alive);
        region.set_cell(-5, 6, Cell::Alive);
        for column in &region.state {
            for cell in column {
                assert_eq!(Cell::Dead, *cell);
            }
        }

        // Inside region
        region.set_cell(5, -5, Cell::Alive);
        assert_eq!(Cell::Alive, region.state[10][0]);
        region.set_cell(-5, 5, Cell::Alive);
        assert_eq!(Cell::Alive, region.state[0][10]);
        region.set_cell(2, -4, Cell::Alive);
        assert_eq!(Cell::Alive, region.state[7][1]);
    }
}



/// Different edges of a region.
pub enum Edge {
    X,
    Y,
    NegX,
    NegY
}