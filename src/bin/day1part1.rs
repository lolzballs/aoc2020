use std::io::{self, BufRead};

use itertools::iproduct;

fn main() {
    let stdin = io::stdin();
    let values: Vec<u64> = {
        let handle = stdin.lock();
        handle
            .lines()
            .map(|f| f.unwrap().parse().unwrap())
            .collect()
    };

    let result = iproduct!(values.iter(), values.iter())
        .filter(|(&a, &b)| a + b == 2020)
        .take(1)
        .map(|(a, b)| a * b)
        .next();

    println!("{:?}", result);
}
