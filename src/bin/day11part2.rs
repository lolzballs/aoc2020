use std::convert::From;
use std::io::BufRead;

type Grid = Vec<Vec<Cell>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Occupied,
    Unoccupied,
    Floor,
}

impl Cell {
    fn occupied(&self) -> bool {
        match self {
            Cell::Occupied => true,
            _ => false,
        }
    }

    fn floor(&self) -> bool {
        match self {
            Cell::Floor => true,
            _ => false,
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Unoccupied,
            '#' => Self::Occupied,
            '.' => Self::Floor,
            _ => panic!(),
        }
    }
}

fn grid_occupied(grid: &Grid, r: isize, c: isize) -> bool {
    if r < 0 {
        false
    } else if r as usize >= grid.len() {
        false
    } else if c < 0 {
        false
    } else if c as usize >= grid[0].len() {
        false
    } else {
        grid[r as usize][c as usize].occupied()
    }
}

fn grid_get(grid: &Grid, r: isize, c: isize) -> Option<Cell> {
    if r < 0 {
        None
    } else if r as usize >= grid.len() {
        None
    } else if c < 0 {
        None
    } else if c as usize >= grid[0].len() {
        None
    } else {
        Some(grid[r as usize][c as usize])
    }
}

fn grid_search(grid: &Grid, r: isize, c: isize) -> [Option<bool>; 8] {
    let mut ret = [None; 8];
    let mut i = 1isize;

    while (0..std::cmp::max(grid.len(), grid[0].len())).contains(&(i as usize)) {
        if ret[0].is_none() {
            match grid_get(grid, r, c + i) {
                Some(Cell::Occupied) => {
                    ret[0] = Some(true);
                }
                Some(Cell::Unoccupied) => {
                    ret[0] = Some(false);
                }
                _ => (),
            }
        }
        if ret[1].is_none() {
            match grid_get(grid, r, c - i) {
                Some(Cell::Occupied) => {
                    ret[1] = Some(true);
                }
                Some(Cell::Unoccupied) => {
                    ret[1] = Some(false);
                }
                _ => (),
            }
        }
        if ret[2].is_none() {
            match grid_get(grid, r + i, c + i) {
                Some(Cell::Occupied) => {
                    ret[2] = Some(true);
                }
                Some(Cell::Unoccupied) => {
                    ret[2] = Some(false);
                }
                _ => (),
            }
        }
        if ret[3].is_none() {
            match grid_get(grid, r + i, c - i) {
                Some(Cell::Occupied) => {
                    ret[3] = Some(true);
                }
                Some(Cell::Unoccupied) => {
                    ret[3] = Some(false);
                }
                _ => (),
            }
        }
        if ret[4].is_none() {
            match grid_get(grid, r - i, c + i) {
                Some(Cell::Occupied) => {
                    ret[4] = Some(true);
                }
                Some(Cell::Unoccupied) => {
                    ret[4] = Some(false);
                }
                _ => (),
            }
        }
        if ret[5].is_none() {
            match grid_get(grid, r - i, c - i) {
                Some(Cell::Occupied) => {
                    ret[5] = Some(true);
                }
                Some(Cell::Unoccupied) => {
                    ret[5] = Some(false);
                }
                _ => (),
            }
        }
        if ret[6].is_none() {
            match grid_get(grid, r + i, c) {
                Some(Cell::Occupied) => {
                    ret[6] = Some(true);
                }
                Some(Cell::Unoccupied) => {
                    ret[6] = Some(false);
                }
                _ => (),
            }
        }
        if ret[7].is_none() {
            match grid_get(grid, r - i, c) {
                Some(Cell::Occupied) => {
                    ret[7] = Some(true);
                }
                Some(Cell::Unoccupied) => {
                    ret[7] = Some(false);
                }
                _ => (),
            }
        }
        i += 1;
    }

    ret
}

fn simulate(in_grid: &Grid, out_grid: &mut Grid) {
    let rows = in_grid.len() as isize;
    let cols = in_grid[0].len() as isize;
    for r in 0..rows {
        for c in 0..cols {
            let sum: usize = grid_search(in_grid, r, c)
                .iter()
                .filter(|&&v| v.unwrap_or(false))
                .count();

            let r = r as usize;
            let c = c as usize;
            out_grid[r][c] = in_grid[r][c];
            match in_grid[r][c] {
                Cell::Unoccupied => {
                    if sum == 0 {
                        out_grid[r][c] = Cell::Occupied
                    }
                }
                Cell::Occupied => {
                    if sum >= 5 {
                        out_grid[r][c] = Cell::Unoccupied
                    }
                }
                _ => (),
            }
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();

    let grid: Vec<Vec<_>> = handle
        .lines()
        .map(|l| l.unwrap().chars().map(Cell::from).collect())
        .collect();

    let mut out_grid = grid.clone();
    let mut in_grid = grid;
    out_grid[0][0] = if out_grid[0][0].occupied() {
        Cell::Unoccupied
    } else {
        Cell::Occupied
    };

    while in_grid != out_grid {
        simulate(&in_grid, &mut out_grid);
        std::mem::swap(&mut in_grid, &mut out_grid);
    }

    let sum: usize = in_grid
        .iter()
        .map(|row| row.iter().filter(|c| c.occupied()).count())
        .sum();
    println!("{}", sum);
}
