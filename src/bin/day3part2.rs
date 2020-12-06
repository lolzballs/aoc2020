use std::io::{self, BufRead};

fn count_trees(lines: &Vec<String>, (right, down): (usize, usize)) -> usize {
    lines
        .iter()
        .step_by(down)
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .cycle()
                .skip(i * right)
                .map(|c| if c == '#' { 1 } else { 0 })
                .next()
                .unwrap()
        })
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let lines: Vec<_> = handle.lines().map(|line| line.unwrap()).collect();
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let product: usize = slopes
        .iter()
        .map(|&slope| count_trees(&lines, slope))
        .product();

    println!("{}", product);
}
