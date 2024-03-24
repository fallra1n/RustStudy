#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let grid: Vec<T> = Vec::with_capacity(rows * cols);
        Self { rows, cols, grid }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        let grid_vec: Vec<T> = grid.to_vec();
        Self {
            rows,
            cols,
            grid: grid_vec,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[self.cols * row + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[self.cols * row + col] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = Vec::new();

        if row != 0 && col != 0 {
            res.push((row - 1, col - 1));
        }

        if row != 0 {
            res.push((row - 1, col));
        }

        if row != 0 && col != self.cols - 1 {
            res.push((row - 1, col + 1));
        }

        if col != 0 {
            res.push((row, col - 1));
        }

        if col != self.cols - 1 {
            res.push((row, col + 1));
        }

        if row != self.rows - 1 && col != 0 {
            res.push((row + 1, col - 1));
        }

        if row != self.rows - 1 {
            res.push((row + 1, col));
        }

        if row != self.rows - 1 && col != self.cols - 1 {
            res.push((row + 1, col + 1));
        }

        res
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let cur_grid: Grid<Cell> = self.grid.clone();

        for i in 0..self.grid.rows {
            for j in 0..self.grid.cols {
                let ns = cur_grid.neighbours(i, j);

                let mut alive_ns: usize = 0;

                for n in ns {
                    match cur_grid.get(n.0, n.1) {
                        Cell::Dead => {}
                        Cell::Alive => {
                            alive_ns += 1;
                        }
                    }
                }

                match cur_grid.get(i, j) {
                    Cell::Dead => {
                        if alive_ns == 3 {
                            self.grid.set(Cell::Alive, i, j);
                        }
                    }

                    Cell::Alive => {
                        if !(2..=3).contains(&alive_ns) {
                            self.grid.set(Cell::Dead, i, j);
                        }
                    }
                }
            }
        }
    }
}
