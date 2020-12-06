use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let lines = handle.lines().map(|line| line.unwrap());
    let count: usize = lines
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .cycle()
                .skip(i * 3)
                .take(1)
                .map(|c| if c == '#' { 1 } else { 0 })
                .next()
                .unwrap()
        })
        .sum();
    println!("{}", count);
}
