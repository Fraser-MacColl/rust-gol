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
        GameOfLife {
            regions: vec![]
        }
    }

    /// Step the simulation to the next state.
    pub fn step(&mut self) {
        !unimplemented!()
        // Step each region
        // Split Regions that have disjoint cells
        // Merge regions that are too close
    }

    /// Check if a position is contained within a region of this world.
    fn pos_in_bounds(&self, x: isize, y: isize) -> bool {
        for region in &self.regions {
            if region.pos_in_bounds(x, y) { return true }
        }
        false
    }

    /// Get the state of the cell at the given x y coordinates.
    pub fn get_cell(&self, x: isize, y: isize) -> Cell {
        for region in &self.regions {
            if let Some(state) = region.get_cell(x, y) {
                return state;
            }
        }
        Cell::Dead
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

#[cfg(test)]
mod game_of_life_tests {
    use super::*;

    #[test]
    fn pos_in_bounds() {
        // TODO
    }

    #[test]
    fn get_cell() {
        // TODO
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
        if x < self.x { return false }
        if y < self.y { return false }

        if let Some(num) = self.x.checked_add_unsigned(self.width) {
            if x >= num { return false }
        }
        else { return false }

        if let Some(num) = self.y.checked_add_unsigned(self.height) {
            if y >= num { return false }
        }
        else { return false }

        // All bound checks passed
        true
    }

    /// Turn world coordinates into local coordinates within this region's internal buffer.
    fn pos_to_local(&self, x: isize, y: isize) -> Option<(usize, usize)> {
        if !self.pos_in_bounds(x, y) { return None }
        Some(((x-self.x) as usize, (y-self.y) as usize))
    }

    /// Returns the state of the cell at the given coordinates.
    /// If the position is outside of this region, returns [`None`].
    pub fn get_cell(&self, x: isize, y: isize) -> Option<Cell> {
        let (x, y) = self.pos_to_local(x, y)?;
        Some(self.state[x][y])
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

    /// Change the size of the region by moving the specified edge.
    /// The amount value is the change in size, not position of the chosen edge.
    /// As such, a positive value even on a negative edge (such as [`Edge::NegX`] or [`Edge::NegY`])
    /// will result in them moving further in the negative direction.
    /// New space is filled with [`Cell::Dead`], while reducing the size truncates the cells.
    /// If adjusting the edges [`Edge::NegX`] or [`Edge::NegY`], the position will be adjusted accordingly.
    pub fn adjust_size(&mut self, edge: Edge, amount: isize) {
        // Adjust size and position values
        match edge {
            Edge::X => self.width = self.width.saturating_add_signed(amount),
            Edge::Y => self.height = self.height.saturating_add_signed(amount),
            Edge::NegX => {
                self.width = self.width.saturating_add_signed(amount);
                self.x -= amount;
            }
            Edge::NegY => {
                self.height = self.height.saturating_add_signed(amount);
                self.y -= amount;
            }
        }

        // Adjust state buffer
        match edge {
            // Add/remove from the end of the outer vec
            Edge::X => {
                self.state.resize(self.width, vec![Cell::Dead; self.height]);
            }

            // Add/remove from the end of each internal vec
            Edge::Y => {
                for column in &mut self.state {
                    column.resize(self.height, Cell::Dead)
                }
            }

            // Add/remove from the start of the outer vec
            Edge::NegX => {
                // Adding extra on the left edge
                if amount >= 0 {
                    self.state.resize(self.width, vec![Cell::Dead]);
                    self.state.as_mut_slice().rotate_right(amount as usize)
                }
                // Removing on the left edge
                else {
                    self.state.as_mut_slice().rotate_left((amount*-1) as usize);
                    self.state.resize(self.width, vec![])
                }
            }

            // Add/remove from the start of the inner vecs
            Edge::NegY => {
                // Adding extra on the bottom edge
                if amount >= 0 {
                    for column in &mut self.state {
                        column.resize(self.height, Cell::Dead);
                        column.as_mut_slice().rotate_right(amount as usize)
                    }
                }
                // Removing on the bottom edge
                else {
                    for column in &mut self.state {
                        column.as_mut_slice().rotate_left((amount*-1) as usize);
                        column.resize(self.width, Cell::Dead)
                    }
                }
            }
        }
    }

    /// Move the region by the given amount in the x and y directions.
    /// New cells will be filled with [`Cell::Dead`], and old cells will be truncated.
    pub fn move_region(&mut self, x: isize, y: isize) {
        // X movement
        self.x += x;
        if x < 0 {
            let x = (x*-1) as usize;
            self.state.as_mut_slice().rotate_right(x);
            for column in &mut self.state[0..x] {
                *column = vec![Cell::Dead; self.height];
            }
        }
        else {
            let x = x as usize;
            self.state.as_mut_slice().rotate_left(x);
            for column in &mut self.state[self.width - x..] {
                *column = vec![Cell::Dead; self.height];
            }
        }

        // Y Movement
        self.y += y;
        if y < 0 {
            // Shadow to avoid duplicate code
            let y = (y*-1) as usize;
            for column in &mut self.state {
                column.as_mut_slice().rotate_right(y);
                column.splice(0..y, vec![Cell::Dead; y]);
            }
        }
        else {
            let y = y as usize;
            for column in &mut self.state {
                column.as_mut_slice().rotate_left(y);
                column.splice((self.height-y).., vec![Cell::Dead; y]);
            }
        }
    }

    // GETTERS
    // Can't just make members public as there are invariants with the vec to maintain.
    pub fn x(&self) -> isize { self.x }
    pub fn y(&self) -> isize { self.y }
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
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
    fn get_cell() {
        // Region going from (-5, -5) up to (5, 5) inclusive
        let region = Region::new(-5, -5, 11, 11);

        // Inbounds
        assert_eq!(Some(Cell::Dead), region.get_cell(-5, -5));
        assert_eq!(Some(Cell::Dead), region.get_cell(5, 5));
        assert_eq!(Some(Cell::Dead), region.get_cell(3, -2));
        assert_eq!(Some(Cell::Dead), region.get_cell(-0, 1));

        // Out of bounds
        assert_eq!(None, region.get_cell(-6, 5));
        assert_eq!(None, region.get_cell(9, 0));
        assert_eq!(None, region.get_cell(5, 6));
        assert_eq!(None, region.get_cell(0, -6));
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

    #[test]
    fn adjust_size() {
        // Region going from (-5, -5) up to (5, 5) inclusive
        let mut region = Region::new(-5, -5, 11, 11);
        // Make all cells alive so we can see the new cells being dead
        for x in -5..=5 {
            for y in -5..=5 {
                region.set_cell(x, y, Cell::Alive)
            }
        }

        // +X edge
        region.adjust_size(Edge::X, 3);
        assert_eq!(14, region.width);
        assert_eq!(14, region.state.len());
        for column in &mut region.state[11..] {
            for cell in column {
                assert_eq!(*cell, Cell::Dead);
                *cell = Cell::Alive // Fill new space with alive cells for following checks
            }
        }

        // +Y edge
        region.adjust_size(Edge::Y, -2);
        assert_eq!(9, region.height);
        for column in &region.state {
            assert_eq!(9, column.len());
            for cell in column {
                assert_eq!(*cell, Cell::Alive)
            }
        }

        // -X edge
        region.adjust_size(Edge::NegX, -2);
        assert_eq!(12, region.width);
        assert_eq!(12, region.state.len());
        for column in &region.state {
            for cell in column {
                assert_eq!(*cell, Cell::Alive)
            }
        }

        // -Y edge
        region.adjust_size(Edge::NegY, 5);
        assert_eq!(14, region.height);
        for column in &region.state {
            assert_eq!(14, column.len());
            for cell in &column[0..5] {
                assert_eq!(*cell, Cell::Dead)
            }
        }
    }

    #[test]
    fn move_region() {
        // Region going from (-5, -5) up to (5, 5) inclusive
        let mut region = Region::new(-5, -5, 11, 11);
        // Make all cells alive so we can see the new cells being dead
        for x in -5..=5 {
            for y in -5..=5 {
                region.set_cell(x, y, Cell::Alive)
            }
        }

        region.move_region(2, 1);
        assert_eq!(-3, region.x);
        assert_eq!(-4, region.y);
        for column in &mut region.state[region.width-2..] {
            for cell in column {
                assert_eq!(Cell::Dead, *cell);
                *cell = Cell::Alive;
            }
        }
        for column in &mut region.state[..region.width-2] {
            for cell in &column[0..region.height-1] {
                assert_eq!(Cell::Alive, *cell);
            }
            assert_eq!(Cell::Dead, *column.last().unwrap());
            *column.last_mut().unwrap() = Cell::Alive;
        }

        region.move_region(-4, -3);
        assert_eq!(-7, region.x);
        assert_eq!(-7, region.y);
        for column in &mut region.state[..4] {
            for cell in column {
                assert_eq!(Cell::Dead, *cell);
                *cell = Cell::Alive;
            }
        }
        for column in &region.state[4..] {
            for cell in &column[..3] {
                assert_eq!(Cell::Dead, *cell);
            }
            for cell in &column[3..] {
                assert_eq!(Cell::Alive, *cell);
            }
        }
    }
}



/// Different edges of a region.
pub enum Edge {
    X,
    Y,
    NegX,
    NegY
}