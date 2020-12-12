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

fn simulate(in_grid: &Grid, out_grid: &mut Grid) {
    let rows = in_grid.len() as isize;
    let cols = in_grid[0].len() as isize;
    for r in 0..rows {
        for c in 0..cols {
            let tl = grid_occupied(in_grid, r - 1, c - 1);
            let tm = grid_occupied(in_grid, r - 1, c);
            let tr = grid_occupied(in_grid, r - 1, c + 1);
            let ml = grid_occupied(in_grid, r, c - 1);
            let mr = grid_occupied(in_grid, r, c + 1);
            let bl = grid_occupied(in_grid, r + 1, c - 1);
            let bm = grid_occupied(in_grid, r + 1, c);
            let br = grid_occupied(in_grid, r + 1, c + 1);

            let sum: usize = [tl, tm, tr, ml, mr, bl, bm, br]
                .iter()
                .filter(|&&v| v)
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
                    if sum >= 4 {
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
